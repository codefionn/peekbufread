use peekbufread::PeekRead;
use std::io::Read;

#[test]
fn peek() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    // Buffered in read, so check peek again
    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[test]
fn peek_partial() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 5] = [0; 5];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(5, result.unwrap());
    assert_eq!(&test[..5], &buf);

    // Buffered in read, so check peek again
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(5, result.unwrap());
    assert_eq!(&test[..5], &buf);
}

#[test]
fn peek_partial_then_all() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 5] = [0; 5];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(5, result.unwrap());
    assert_eq!(&test[..5], &buf);

    // Buffered in read, so check peek again
    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[test]
fn peek_all_read_partial() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..12], &buf);
}

#[test]
fn read_partial_then_peek() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());
    let result = read.read(&mut [0; 2]);
    assert!(result.is_ok());
    assert_eq!(2, result.unwrap());

    let mut buf: [u8; 9] = [0; 9];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(9, result.unwrap());
    assert_eq!(&test[2..11], buf);
}

#[test]
fn peek_read() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(0, result.unwrap());
}

#[test]
fn partial_peek_peek_read() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(0, result.unwrap());
}

#[test]
fn partial_peek_read_read() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..], &buf);

    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(0, result.unwrap());
}

#[test]
fn peek_partial_read_all() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(0, result.unwrap());
}

#[test]
fn peek_partial_read_peek() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..12], &buf);

    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..12], &buf);
}

#[test]
fn peek_read_peek() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(&test[..12], &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..12], &buf);

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..12], &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_read_ok() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 5] = [0; 5];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(5, result.unwrap());
        assert_eq!(&test[..5], &buf);

        return Ok(());
    })
    .ok();

    let mut buf: [u8; 7] = [0; 7];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(7, result.unwrap());
    assert_eq!(&test[5..], &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_read_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 12] = [0; 12];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(12, result.unwrap());
        assert_eq!(&test[..12], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_read_partial_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[..6], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_peek_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 12] = [0; 12];
        let result = read.peek(&mut buf);
        assert!(result.is_ok());
        assert_eq!(12, result.unwrap());
        assert_eq!(&test[..12], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_peek_partial_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 6] = [0; 6];
        let result = read.peek(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[..6], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn read_partial_checkpoint_read_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[6..], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..], &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn read_partial_checkpoint_read_peek_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 6] = [0; 6];
        let result = read.peek(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[6..], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 6] = [0; 6];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[6..], &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_ok_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 6] = [0; 6];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(6, result.unwrap());
            assert_eq!(&test[..6], &buf);

            return Ok(());
        })
        .ok();

        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[6..], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn partial_peek_checkpoint_ok_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 4] = [0; 4];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(4, result.unwrap());
    assert_eq!(&test[..4], &buf);

    read.checkpoint(|read| -> Result<(), ()> {
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 6] = [0; 6];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(6, result.unwrap());
            assert_eq!(&test[4..10], &buf);

            return Ok(());
        })
        .ok();

        let mut buf: [u8; 2] = [0; 2];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(2, result.unwrap());
        assert_eq!(&test[10..], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 8] = [0; 8];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(8, result.unwrap());
    assert_eq!(&test[4..], &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_ok_after_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[..6], &buf);

        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 6] = [0; 6];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(6, result.unwrap());
            assert_eq!(&test[6..], &buf);

            return Ok(());
        })
        .ok();

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn partial_peek_checkpoint_err_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());
    let mut buf: [u8; 4] = [0; 4];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(4, result.unwrap());
    assert_eq!(&test[..4], &buf);

    read.checkpoint(|read| -> Result<(), ()> {
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 4] = [0; 4];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(4, result.unwrap());
            assert_eq!(&test[4..8], &buf);

            return Err(());
        })
        .ok();

        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[4..10], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 8] = [0; 8];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(8, result.unwrap());
    assert_eq!(&test[4..], &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_err_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 4] = [0; 4];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(4, result.unwrap());
            assert_eq!(&test[..4], &buf);

            return Err(());
        })
        .ok();

        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[..6], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_err_equal_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 6] = [0; 6];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(6, result.unwrap());
            assert_eq!(&test[..6], &buf);

            return Err(());
        })
        .ok();

        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[..6], &buf);

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[cfg(feature = "checkpoint")]
#[test]
fn checkpoint_err_after_in_checkpoint_err() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    read.checkpoint(|read| -> Result<(), ()> {
        let mut buf: [u8; 6] = [0; 6];
        let result = read.read(&mut buf);
        assert!(result.is_ok());
        assert_eq!(6, result.unwrap());
        assert_eq!(&test[..6], &buf);

        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf: [u8; 4] = [0; 4];
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(4, result.unwrap());
            assert_eq!(&test[6..10], &buf);

            return Err(());
        })
        .ok();

        return Err(());
    })
    .ok();

    let mut buf: [u8; 12] = [0; 12];
    let result = read.read(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);
}

#[test]
fn peek_read_until() {
    let test = b"hello, world";
    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 12] = [0; 12];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(12, result.unwrap());
    assert_eq!(test, &buf);

    use std::io::BufRead;

    let mut buf = Vec::new();
    let result = read.read_until(b',', &mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);
}

#[test]
fn partial_peek_read_until() {
    let test = b"hello, world";

    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 8] = [0; 8];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(8, result.unwrap());
    assert_eq!(&test[..8], &buf);

    use std::io::BufRead;

    let mut buf = Vec::new();
    let result = read.read_until(b',', &mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 6] = [0; 6];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut buf = Vec::new();
    let result = read.read_until(b',', &mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);

    let mut read = PeekRead::new(test.as_ref());

    let mut buf: [u8; 5] = [0; 5];
    let result = read.peek(&mut buf);
    assert!(result.is_ok());
    assert_eq!(5, result.unwrap());
    assert_eq!(&test[..5], &buf);

    let mut buf = Vec::new();
    let result = read.read_until(b',', &mut buf);
    assert!(result.is_ok());
    assert_eq!(6, result.unwrap());
    assert_eq!(&test[..6], &buf);
}
