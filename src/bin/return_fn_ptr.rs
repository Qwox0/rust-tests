#![feature(test)]

use std::{iter::Map, ops::Range};

pub fn square(x: i32) -> i32 {
    x * x
}

/// test bench::bench_f1 ... bench:           3 ns/iter (+/- 0)
#[inline(never)]
pub fn f1(n: i32) -> impl Iterator<Item = i32> {
    (0..n).map(square)
}

/// asm is identical to [`f1`]
///
/// test bench::bench_f2 ... bench:           3 ns/iter (+/- 0)
#[inline(never)]
pub fn f2(n: i32) -> Map<Range<i32>, impl FnMut(i32) -> i32> {
    (0..n).map(square)
}

/// don't use this
///
/// test bench::bench_f3 ... bench:   1,289,022 ns/iter (+/- 44,132)
#[inline(never)]
pub fn f3(n: i32) -> Map<Range<i32>, fn(i32) -> i32> {
    (0..n).map(square)
}

pub fn main() {
    println!("f1(5).sum(): {}", f1(5).sum::<i32>());
    println!("f2(5).sum(): {}", f2(5).sum::<i32>());
    println!("f3(5).sum(): {}", f3(5).sum::<i32>());
}

#[cfg(test)]
mod bench {
    extern crate test;

    use crate::*;
    use test::*;

    const IN: i32 = 1_000_000;

    macro_rules! make_bench {
        ($bench_name:ident : $fn:ident) => {
            #[bench]
            fn $bench_name(b: &mut Bencher) {
                b.iter(|| {
                    black_box(black_box($fn(black_box(IN))).sum::<i32>());
                })
            }
        };
    }

    make_bench! { bench_f1: f1 }
    make_bench! { bench_f2: f2 }
    make_bench! { bench_f3: f3 }
}
