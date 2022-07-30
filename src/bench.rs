use crate::PeekRead;
use rand::prelude::*;
use std::io::Cursor;
use test::Bencher;

const LEN: usize = 1024;

fn generate_1m() -> Box<[u8; LEN * 1024]> {
    let mut result = Box::new([0; LEN * 1024]);

    let mut rng = thread_rng();
    for i in 0..1024 {
        rng.fill(&mut result[i * LEN..(i + 1) * LEN]);
    }

    return result;
}

#[bench]
fn read_exact_normal(bench: &mut Bencher) {
    let data = generate_1m();

    bench.iter(move || {
        let mut read = Cursor::new(data.as_ref());
        let mut buf = [0; LEN];
        for _ in 0..data.len() / 1024 {
            use std::io::Read;
            // Don't optimize me away
            let result = read.read_exact(&mut buf);
            assert!(result.is_ok());
            test::black_box(result).unwrap();
        }
    });
}

#[bench]
fn read_exact_normal_peekread(bench: &mut Bencher) {
    let data = generate_1m();

    bench.iter(move || {
        let mut read = PeekRead::new(Cursor::new(data.as_ref()));
        let mut buf = [0; LEN];
        for _ in 0..data.len() / 1024 {
            use std::io::Read;
            // Don't optimize me away
            let result = read.read_exact(&mut buf);
            assert!(result.is_ok());
            test::black_box(result).unwrap();
        }
    });
}

#[cfg(feature = "checkpoint")]
#[bench]
fn read_exact_with_checkpoint_peekread(bench: &mut Bencher) {
    let data = generate_1m();

    bench.iter(move || {
        let mut read = PeekRead::new(Cursor::new(data.as_ref()));
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf = [0; LEN];
            for _ in 0..data.len() / 1024 {
                use std::io::Read;
                let result = read.read_exact(&mut buf);
                assert!(result.is_ok());
                test::black_box(result).unwrap();
            }

            Ok(())
        })
        .unwrap();
    });
}

#[bench]
fn read_normal(bench: &mut Bencher) {
    let data = generate_1m();

    bench.iter(move || {
        let mut read = Cursor::new(data.as_ref());
        let mut buf = [0; LEN];
        for _ in 0..data.len() / 1024 {
            use std::io::Read;
            // Don't optimize me away
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(LEN, result.unwrap());
        }
    });
}

#[bench]
fn read_normal_peekread(bench: &mut Bencher) {
    let data = generate_1m();

    bench.iter(move || {
        let mut read = PeekRead::new(Cursor::new(data.as_ref()));
        let mut buf = [0; LEN];
        for _ in 0..data.len() / 1024 {
            use std::io::Read;
            // Don't optimize me away
            let result = read.read(&mut buf);
            assert!(result.is_ok());
            assert_eq!(LEN, result.unwrap());
        }
    });
}

#[cfg(feature = "checkpoint")]
#[bench]
fn read_with_checkpoint_peekread(bench: &mut Bencher) {
    let data = generate_1m();

    bench.iter(move || {
        let mut read = PeekRead::new(Cursor::new(data.as_ref()));
        read.checkpoint(|read| -> Result<(), ()> {
            let mut buf = [0; LEN];
            for _ in 0..data.len() / 1024 {
                use std::io::Read;
                let result = read.read(&mut buf);
                assert!(result.is_ok());
                assert_eq!(LEN, result.unwrap());
            }

            Ok(())
        })
        .unwrap();
    });
}
