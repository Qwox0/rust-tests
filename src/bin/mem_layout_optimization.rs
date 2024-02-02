use std::{mem::size_of, num::NonZeroUsize};

pub enum CustomOption {
    Num(NonZeroUsize),
    Other,
}

pub enum E {
    A,
    B,
    C,
    D,
}

fn main() {
    println!("                bool: {}", size_of::<bool>());
    println!("                  u8: {}", size_of::<u8>());
    println!("               usize: {}", size_of::<usize>());
    println!("       Option<usize>: {}", size_of::<Option<usize>>());
    println!("        NonZeroUsize: {}", size_of::<NonZeroUsize>());
    println!("Option<NonZeroUsize>: {}", size_of::<Option<NonZeroUsize>>());
    println!("        CustomOption: {}", size_of::<CustomOption>());
    println!("                   E: {}", size_of::<E>());
}
