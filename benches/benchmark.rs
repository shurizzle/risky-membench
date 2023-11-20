use std::mem::MaybeUninit;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use membench::{
    new::{self, U32},
    old,
};

const HELL: u32 = 1819043176; // b"hell"

#[inline(always)]
fn into_mem<const N: usize, R, F>(f: F) -> R
where
    F: Fn(&mut [u8]) -> R,
{
    let mut mem = MaybeUninit::<[u8; N]>::uninit();
    f(black_box(unsafe { &mut *mem.as_mut_ptr() }.as_mut_slice()))
}

fn bench_u32_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("write u32");
    group.bench_function("old", |b| {
        b.iter(|| {
            into_mem::<4, _, _>(|mem| {
                old::memw(&black_box(HELL).to_le_bytes(), mem, 0).unwrap();
                assert_eq!(mem, b"hell");
            })
        })
    });
    group.bench_function("new", |b| {
        b.iter(|| {
            into_mem::<4, _, _>(|mem| {
                new::write(&U32::new(black_box(HELL)), mem, 0).unwrap();
                assert_eq!(mem, b"hell");
            })
        })
    });
    group.finish();
}

fn bench_slice_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("write slice");
    group.bench_function("old", |b| {
        b.iter(|| {
            into_mem::<4, _, _>(|mem| {
                old::memw(b"hell", mem, 0).unwrap();
                assert_eq!(mem, b"hell");
            })
        })
    });
    group.bench_function("new", |b| {
        b.iter(|| {
            into_mem::<4, _, _>(|mem| {
                new::write(b"hell", mem, 0).unwrap();
                assert_eq!(mem, b"hell");
            })
        })
    });
    group.finish();
}

fn bench_u32_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("read u32");
    group.bench_function("old", |b| {
        b.iter(|| {
            let mem = black_box(b"hello_world".as_slice());
            assert_eq!(u32::from_le_bytes(old::memr32(mem, 0).unwrap()), HELL);
        })
    });
    group.bench_function("new", |b| {
        b.iter(|| {
            let mem = black_box(b"hello_world".as_slice());
            assert_eq!(new::read::<U32>(mem, 0).unwrap().as_u32(), HELL);
        })
    });
    group.finish();
}

fn bench_slice_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("read slice");
    group.bench_function("old", |b| {
        b.iter(|| {
            let mem = black_box(b"hello_world".as_slice());
            assert_eq!(old::memr32(mem, 0).unwrap(), *b"hell");
        })
    });
    group.bench_function("new", |b| {
        b.iter(|| {
            let mem = black_box(b"hello_world".as_slice());
            assert_eq!(new::memr32(mem, 0).unwrap(), *b"hell");
        })
    });
    group.finish();
}

criterion_group!(bench_writes, bench_u32_write, bench_slice_write);
criterion_group!(bench_reads, bench_u32_read, bench_slice_read);
criterion_main!(bench_writes, bench_reads);
