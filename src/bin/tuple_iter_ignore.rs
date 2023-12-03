#![feature(test)]

use std::ops::Range;

const RANGE: Range<i32> = 1..10000;

fn opt(range: Range<i32>) -> impl Iterator<Item = i32> {
    range.map(|x| {
        let x = get_int(x);
        x
    })
}

fn ignore(range: Range<i32>) -> impl Iterator<Item = i32> {
    both(range).map(|(x, _)| x)
}

fn both(range: Range<i32>) -> impl Iterator<Item = (i32, f64)> {
    range.map(|x| {
        let x = get_int(x);
        let f = get_float(x);
        (x, f)
    })
}

fn ignore_inline(range: Range<i32>) -> impl Iterator<Item = i32> {
    range
        .map(|x| {
            let x = get_int(x);
            let f = get_float(x);
            (x, f)
        })
        .map(|(x, _)| x)
}

fn get_int(x: i32) -> i32 {
    69 - x
}

fn get_float(x: i32) -> f64 {
    x as f64 / 1.6180339887
}

pub fn main() {
    let a: i32 = opt(RANGE).sum();
    let b: i32 = ignore(RANGE).sum();

    assert_eq!(a, b);
}

#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::*;

    fn bench_iter(iter: impl Iterator<Item = i32>) {
        iter.sum::<i32>();
    }

    /// ## bench
    ///
    /// test benches::bench_opt    ... bench:           0 ns/iter (+/- 0)
    ///
    /// ## bench --profile=dev
    ///
    /// test benches::bench_opt    ... bench:     215,893 ns/iter (+/- 52,321)
    #[bench]
    fn bench_opt(b: &mut Bencher) {
        b.iter(|| black_box(bench_iter(black_box(opt(black_box(RANGE))))))
    }

    /// ## bench
    ///
    /// test benches::bench_ignore ... bench:           0 ns/iter (+/- 0)
    ///
    /// ## bench --profile=dev
    ///
    /// test benches::bench_ignore ... bench:     336,823 ns/iter (+/- 9,049)
    #[bench]
    fn bench_ignore(b: &mut Bencher) {
        b.iter(|| black_box(bench_iter(black_box(ignore(black_box(RANGE))))))
    }

    /// ## bench
    ///
    /// test benches::bench_ignore_inline ... bench:           0 ns/iter (+/- 0)
    ///
    /// ## bench --profile=dev
    ///
    /// test benches::bench_ignore_inline ... bench:     337,100 ns/iter (+/- 31,124)
    #[bench]
    fn bench_ignore_inline(b: &mut Bencher) {
        b.iter(|| black_box(bench_iter(black_box(ignore_inline(black_box(RANGE))))))
    }
}
