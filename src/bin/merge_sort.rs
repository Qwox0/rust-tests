#![feature(let_chains)]
#![feature(test)]
#![feature(is_sorted)]
#![feature(get_many_mut)]

use rand::seq::SliceRandom;
use std::fmt::Debug;

fn merge_sort_iter<T: Copy + PartialOrd + Debug>(arr: &[T]) -> Vec<T> {
    fn merge<T: Copy + PartialOrd + Debug>(l: &[T], r: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(l.len() + r.len());
        let mut l_iter = l.into_iter();
        let mut r_iter = r.into_iter();

        let mut l_item = l_iter.next();
        let mut r_item = r_iter.next();

        while let Some(l) = l_item && let Some(r) = r_item {
            if l > r {
                out.push(*r);
                r_item = r_iter.next();
            } else {
                out.push(*l);
                l_item = l_iter.next();
            }
        }
        out.extend(l_item);
        out.extend(r_item);
        out.extend(l_iter);
        out.extend(r_iter);
        out
    }

    let len = arr.len();

    if len == 1 {
        return arr.to_vec()
    }

    let mid = len / 2;
    merge(&merge_sort_iter(&arr[..mid]), &merge_sort_iter(&arr[mid..]))
}

fn merge_sort_idx<T: Copy + PartialOrd + Debug>(arr: &[T]) -> Vec<T> {
    fn merge<T: Copy + PartialOrd + Debug>(l: &[T], r: &[T]) -> Vec<T> {
        let mut out = Vec::with_capacity(l.len() + r.len());
        let mut l_idx = 0;
        let mut r_idx = 0;

        while let Some(l) = l.get(l_idx) && let Some(r) = r.get(r_idx){
            if l > r {
                out.push(*r);
                r_idx += 1;
            } else {
                out.push(*l);
                l_idx += 1;
            }
        }
        out.extend_from_slice(&l[l_idx..]);
        out.extend_from_slice(&r[r_idx..]);
        out
    }

    let len = arr.len();

    if len == 1 {
        return arr.to_vec()
    }

    let mid = len / 2;

    merge(&merge_sort_idx(&arr[..mid]), &merge_sort_idx(&arr[mid..]))
}

fn get_rand_vec(len: usize) -> Vec<usize> {
    let mut vec = (0..len).collect::<Vec<_>>();
    vec.shuffle(&mut rand::thread_rng());
    vec
}

fn merge_sort_idx_inline<T: Copy + PartialOrd + Debug>(arr: &[T]) -> Vec<T> {
    let len = arr.len();
    if len == 1 {
        return arr.to_vec()
    }

    let mid = len / 2;

    let l = &merge_sort_idx_inline(&arr[..mid]);
    let r = &merge_sort_idx_inline(&arr[mid..]);

    let mut out = Vec::with_capacity(len);
    let mut l_idx = 0;
    let mut r_idx = 0;

    while let Some(l) = l.get(l_idx) && let Some(r) = r.get(r_idx) {
        if l > r {
            out.push(*r);
            r_idx += 1;
        } else {
            out.push(*l);
            l_idx += 1;
        }
    }
    out.extend_from_slice(&l[l_idx..]);
    out.extend_from_slice(&r[r_idx..]);
    out
}

fn merge_sort_manual_push<T: Copy + PartialOrd + Debug>(arr: &[T]) -> Vec<T> {
    let len = arr.len();

    match len.cmp(&2) {
        std::cmp::Ordering::Equal => {
            let Some(&[a, b]) = arr.get(..) else { unreachable!() };
            return if b < a { vec![b, a] } else { arr.to_vec() }
        },
        std::cmp::Ordering::Less => return arr.to_vec(),
        _ => (),
    }

    let mid = len / 2;

    let l = &merge_sort_manual_push(&arr[..mid]);
    let r = &merge_sort_manual_push(&arr[mid..]);

    let mut out = Vec::with_capacity(len);
    let mut l_idx = 0;
    let mut r_idx = 0;

    while let Some(&l) = l.get(l_idx) && let Some(&r) = r.get(r_idx) {
        out.push(if l > r {
            r_idx += 1;
            r
        } else {
            l_idx += 1;
            l
        })
    }

    for idx in l_idx..mid {
        out.push(l[idx]);
    }

    for idx in r_idx..(len - mid) {
        out.push(r[idx]);
    }

    out
}

fn merge_sort_memcpy<T: Copy + PartialOrd + Debug + Default>(arr: &[T]) -> Vec<T> {
    let len = arr.len();

    match len.cmp(&2) {
        std::cmp::Ordering::Equal => {
            let Some(&[a, b]) = arr.get(..) else { unreachable!() };
            return if b < a { vec![b, a] } else { arr.to_vec() }
        },
        std::cmp::Ordering::Less => return arr.to_vec(),
        _ => (),
    }

    let mid = len / 2;
    let (l, r) = arr.split_at(mid);
    let l = merge_sort_memcpy(l);
    let r = merge_sort_memcpy(r);

    let mut out = Vec::with_capacity(len);
    // SAFETY: uninitialized content of `out` will be overwritten
    unsafe { out.set_len(len) }
    let mut out_idx = 0;
    let mut l_idx = 0;
    let mut r_idx = 0;

    while let Some(&l) = l.get(l_idx) && let Some(&r) = r.get(r_idx) {
        out[out_idx] = if l > r {
            r_idx += 1;
            r
        } else {
            l_idx += 1;
            l
        };
        out_idx += 1;
    }

    let dst = unsafe { out.as_mut_ptr().add(out_idx) };

    let l_rem = l[l_idx..].as_ptr();
    let lcount = (l_idx..mid).len();
    unsafe { std::ptr::copy_nonoverlapping(l_rem, dst, lcount) }

    let r_rem = r[r_idx..].as_ptr();
    let rcount = (r_idx..(len - mid)).len();
    unsafe { std::ptr::copy_nonoverlapping(r_rem, dst.add(lcount), rcount) }

    out
}

fn merge_sort<T: Copy + PartialOrd + Debug + Default>(arr: &[T]) -> Vec<T> {
    let len = arr.len();

    /*
    if let &[a, b] = arr {
        return if b < a { vec![b, a] } else { arr.to_vec() }
    } else if len < 2 {
        return arr.to_vec()
    }
    */

    /*
    match arr {
        &[a, b] => return if b < a { vec![b, a] } else { arr.to_vec() },
        arr if len < 2 => return arr.to_vec(),
        _ => (),
    }
    */

    match len.cmp(&2) {
        std::cmp::Ordering::Equal => {
            let Some(&[a, b]) = arr.get(..) else { unreachable!() };
            return if b < a { vec![b, a] } else { arr.to_vec() }
        },
        std::cmp::Ordering::Less => return arr.to_vec(),
        _ => (),
    }

    let mid = len / 2;
    let (l, r) = arr.split_at(mid);
    let l = merge_sort(l);
    let r = merge_sort(r);

    let mut out = Vec::with_capacity(len);
    // SAFETY: uninitialized content of `out` will be overwritten
    unsafe { out.set_len(len) }
    let mut out_idx = 0;
    let mut l_idx = 0;
    let mut r_idx = 0;

    while let Some(&l) = l.get(l_idx) && let Some(&r) = r.get(r_idx) {
        out[out_idx] = if l > r {
            r_idx += 1;
            r
        } else {
            l_idx += 1;
            l
        };
        out_idx += 1;
    }

    /*
    // why is this slower?
    for x in &l[l_idx..] {
        out[out_idx] = *x;
        out_idx += 1;
    }

    for x in &r[r_idx..] {
        out[out_idx] = *x;
        out_idx += 1;
    }
    */

    for idx in l_idx..mid {
        out[out_idx] = l[idx];
        out_idx += 1;
    }

    for idx in r_idx..(len - mid) {
        out[out_idx] = r[idx];
        out_idx += 1;
    }

    out
}

fn main() {
    let vec = get_rand_vec(20);
    print!("{:?}", vec);
    let vec = merge_sort_iter(&vec);
    println!(" -> {:?}", vec);
    assert!(vec.is_sorted());

    let vec = get_rand_vec(20);
    print!("{:?}", vec);
    let vec = merge_sort_idx(&vec);
    println!(" -> {:?}", vec);
    assert!(vec.is_sorted());

    let vec = get_rand_vec(20);
    print!("{:?}", vec);
    let vec = merge_sort_idx_inline(&vec);
    println!(" -> {:?}", vec);
    assert!(vec.is_sorted());

    let vec = get_rand_vec(20);
    print!("{:?}", vec);
    let vec = merge_sort_manual_push(&vec);
    println!(" -> {:?}", vec);
    assert!(vec.is_sorted());

    let vec = get_rand_vec(20);
    print!("{:?}", vec);
    let vec = merge_sort_memcpy(&vec);
    println!(" -> {:?}", vec);
    assert!(vec.is_sorted());

    let vec = get_rand_vec(20);
    print!("{:?}", vec);
    let vec = merge_sort(&vec);
    println!(" -> {:?}", vec);
    assert!(vec.is_sorted());
}

#[cfg(test)]
mod benches {
    extern crate test;
    use super::*;
    use test::*;

    const TEST_LEN: usize = 10000;

    #[bench]
    fn test_iter(b: &mut Bencher) {
        let vec = get_rand_vec(TEST_LEN);
        b.iter(|| {
            let out = black_box(merge_sort_iter(black_box(&vec)));
            out
        });
    }

    #[bench]
    fn test_idx(b: &mut Bencher) {
        let vec = get_rand_vec(TEST_LEN);
        b.iter(|| {
            let out = black_box(merge_sort_idx(black_box(&vec)));
            out
        });
    }

    #[bench]
    fn test_idx_inline(b: &mut Bencher) {
        let vec = get_rand_vec(TEST_LEN);
        b.iter(|| {
            let out = black_box(merge_sort_idx_inline(black_box(&vec)));
            out
        });
    }

    #[bench]
    fn test_manual_push(b: &mut Bencher) {
        let vec = get_rand_vec(TEST_LEN);
        b.iter(|| {
            let out = black_box(merge_sort_manual_push(black_box(&vec)));
            out
        });
    }

    #[bench]
    fn test_memcpy0(b: &mut Bencher) {
        let vec = get_rand_vec(TEST_LEN);
        b.iter(|| {
            let out = black_box(merge_sort_memcpy(black_box(&vec)));
            out
        });
    }

    #[bench]
    fn test_final0(b: &mut Bencher) {
        let vec = get_rand_vec(TEST_LEN);
        b.iter(|| {
            let out = black_box(merge_sort(black_box(&vec)));
            out
        });
    }
}
