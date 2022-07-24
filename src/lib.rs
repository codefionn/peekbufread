#![feature(test)]
#![feature(core_intrinsics)]

#[cfg(test)]
extern crate test;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_exact;

#[cfg(test)]
mod bench;

use std::intrinsics::unlikely;

/// Peekable for IO-read. Works by buffering peeked data.
/// Also supports checkpoints.
pub struct PeekRead<Read: std::io::Read> {
    ioread: Read,
    buffer: Vec<u8>,
    pos: Option<usize>,
    checkpoints: Vec<usize>,
}

impl<Read: std::io::Read> PeekRead<Read> {
    #[inline]
    fn consume_buffer_bytes(&mut self, bytes: usize) {
        if let Some(pos) = self.pos {
            if bytes >= self.buffer.len() {
                if self.checkpoints.is_empty() {
                    self.pos = None;
                    self.buffer.clear();
                } else {
                    self.pos = Some(0);
                }
            } else {
                // Magic size (where to empty data)
                if pos >= 128 && self.checkpoints.is_empty() {
                    for _ in 0..=bytes {
                        self.buffer.remove(0);
                    }
                    self.pos = Some(0);
                } else {
                    self.pos = Some(pos + bytes);
                }
            }
        }
    }

    pub fn new(read: Read) -> Self {
        Self {
            ioread: read,
            buffer: Vec::new(),
            pos: None,
            checkpoints: Vec::new(),
        }
    }

    /// Peeks the stream
    ///
    /// Returns the top of the stream without consuming its contents
    #[must_use]
    pub fn peek(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(pos) = self.pos {
            let consumed = buf.len().min(self.buffer.len() - pos);
            if consumed != 0 {
                buf[..consumed].copy_from_slice(&self.buffer[pos..(consumed + pos)]);
            }

            if consumed == buf.len() {
                return Ok(consumed);
            } else {
                let consumed_reader = self.ioread.read(&mut buf[consumed..])?;
                self.buffer
                    .extend(buf[consumed..consumed + consumed_reader].iter());
                return Ok(consumed + consumed_reader);
            }
        } else {
            let result = self.ioread.read(buf)?;
            self.buffer.extend(buf[..result].iter());
            self.pos = Some(0);
            return Ok(result);
        }
    }

    /// Peeks the stream
    ///
    /// Returns the top of the stream without consuming its contents
    #[must_use]
    pub fn peek_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if let Some(pos) = self.pos {
            let consumed = buf.len().min(self.buffer.len() - pos);
            if consumed != 0 {
                buf[..consumed].copy_from_slice(&self.buffer[pos..(consumed + pos)]);
            }

            if consumed == buf.len() {
                return Ok(());
            } else {
                self.ioread.read_exact(&mut buf[consumed..])?;
                self.buffer.extend(buf[consumed..].iter());
                return Ok(());
            }
        } else {
            self.ioread.read_exact(buf)?;
            self.buffer.extend(buf.iter());
            self.pos = Some(0);
            return Ok(());
        }
    }

    /// Creates a checkpoint and calls fn_checkpoint afterwards
    ///
    /// Resets the reader to the current state if an Error is returned. Can be stacked.
    #[must_use]
    pub fn checkpoint<T, E, F: FnOnce(&mut Self) -> Result<T, E>>(
        &mut self,
        fn_checkpoint: F,
    ) -> Result<T, E> {
        self.checkpoints.push(self.pos.unwrap_or(0));

        let result = fn_checkpoint(self);
        return match result {
            Ok(result) => {
                self.checkpoints.pop();
                Ok(result)
            }
            Err(err) => {
                self.pos = Some(self.checkpoints.pop().unwrap());
                Err(err)
            }
        };
    }

    fn read_with_pos(&mut self, buf: &mut [u8], pos: usize) -> std::io::Result<usize> {
        let consumed = buf.len().min(self.buffer.len() - pos);
        if consumed != 0 {
            buf[..consumed].copy_from_slice(&self.buffer[pos..(consumed + pos)]);
            self.consume_buffer_bytes(consumed);
        }

        if consumed == buf.len() {
            // The consumed bytes were all buffered already
            return Ok(consumed);
        } else {
            // The consumed bytes were only partially buffered
            let consumed_reader = self.ioread.read(&mut buf[consumed..])?;
            if !self.checkpoints.is_empty() {
                self.buffer.extend(buf[consumed..].iter());
                self.pos = Some(self.buffer.len());
            }
            return Ok(consumed + consumed_reader);
        }
    }

    fn read_exact_with_pos(&mut self, buf: &mut [u8], pos: usize) -> std::io::Result<()> {
        let consumed = buf.len().min(self.buffer.len() - pos);
        if consumed != 0 {
            buf[..consumed].copy_from_slice(&self.buffer[pos..(consumed + pos)]);
            self.consume_buffer_bytes(consumed);
        }

        if consumed == buf.len() {
            // The consumed bytes were all buffered already
            return Ok(());
        } else {
            // The consumed bytes were only partially buffered
            self.ioread.read_exact(&mut buf[consumed..])?;
            if !self.checkpoints.is_empty() {
                self.buffer.extend(buf[consumed..].iter());
                self.pos = Some(self.buffer.len());
            }
            return Ok(());
        }
    }
}

impl<Read: std::io::Read> std::io::Read for PeekRead<Read> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(pos) = self.pos {
            return self.read_with_pos(buf, pos);
        } else {
            let result = self.ioread.read(buf)?;
            if unlikely(!self.checkpoints.is_empty()) {
                self.buffer.extend(buf[..result].iter());
                self.pos = Some(result);
            }
            return Ok(result);
        }
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if let Some(pos) = self.pos {
            return self.read_exact_with_pos(buf, pos);
        } else {
            self.ioread.read_exact(buf)?;
            if unlikely(!self.checkpoints.is_empty()) {
                self.buffer.extend(buf.iter());
                self.pos = Some(self.buffer.len());
            }
            return Ok(());
        }
    }
}

impl<Read: std::io::BufRead> std::io::BufRead for PeekRead<Read> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if let Some(pos) = self.pos {
            if self.buffer.len() > pos {
                return Ok(&self.buffer[pos..]);
            } else {
                return self.ioread.fill_buf();
            }
        } else {
            return self.ioread.fill_buf();
        }
    }

    fn consume(&mut self, amt: usize) {
        if let Some(pos) = self.pos {
            if amt > self.buffer.len() - pos {
                self.consume_buffer_bytes(self.buffer.len() - pos);
                self.ioread.consume(amt - (self.buffer.len() - pos));
            } else {
                self.consume_buffer_bytes(amt);
            }
        } else {
            self.ioread.consume(amt);
        }
    }
}
