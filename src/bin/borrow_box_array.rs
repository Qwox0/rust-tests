use std::{
    borrow::{Borrow, BorrowMut},
    ops::Deref,
};

#[derive(Debug)]
struct HeapArray<const N: usize>(Box<[f64; N]>);

impl<const N: usize> Borrow<[f64; N]> for HeapArray<N> {
    fn borrow(&self) -> &[f64; N] {
        &self.0
    }
}

impl<const N: usize> BorrowMut<[f64; N]> for HeapArray<N> {
    fn borrow_mut(&mut self) -> &mut [f64; N] {
        &mut self.0
    }
}

impl<const I: usize> Deref for HeapArray<I> {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

fn print_arr<const N: usize>(arr: &impl Borrow<[f64; N]>) {
    let arr = arr.borrow();
    println!("{:?}", arr);
}

fn make_increasing<const N: usize>(arr: &mut impl BorrowMut<[f64; N]>) {
    for (idx, x) in arr.borrow_mut().iter_mut().enumerate() {
        *x = idx as f64;
    }
}

fn main() {
    let mut arr = [1.0; 5];

    let mut heap_arr: HeapArray<5> = HeapArray(vec![2.0; 5].try_into().unwrap());

    make_increasing(&mut arr);
    make_increasing(&mut heap_arr);

    print_arr(&arr);
    print_arr(&heap_arr);
}
