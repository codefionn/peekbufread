# peekbufread

Allows to peek data of abitrary `std::io::Read` and comes with supports for
checkpoints. Both features work by buffering parts of the original stream.

This crate is intentionally kept very simple: it only offers the struct
PeekRead and nothing on top of it.

## Build & test

```bash
git clone https://github.com/codefionn/peekbufread.git
cd peekbufread
cargo test
cargo bench
```

## Peek

Allows to peek data without consuming its contents.

## Checkpoints

Allows the program to forget that data of a stream was already read.

Checkpoint support is optional, but included by default, disable by

```
peekbufread = { version = "*", default-features = false }
```

This makes the performance of the crate faster (this issue is
currently under investigation).
