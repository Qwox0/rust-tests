#![feature(iter_array_chunks)]

use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitOr, BitXor},
};

/// [`W`]: Bitmaskwith in bytes
#[derive(Clone, Copy)]
pub struct Bitmask<const W: usize> {
    mask: [u8; W],
}

impl<const W: usize> Bitmask<W> {
    pub const fn default() -> Bitmask<W> {
        Bitmask { mask: [0; W] }
    }

    pub const fn from_bytes(bytes: [u8; W]) -> Bitmask<W> {
        Bitmask { mask: bytes }
    }
}

macro_rules! impl_operations {
    ($($op:ident: $method:ident),*) => {
        $(
            impl<const W: usize> $op<Bitmask<W>> for Bitmask<W> {
                type Output = Bitmask<W>;

                fn $method(mut self, rhs: Bitmask<W>) -> Self::Output {
                    self.mask.iter_mut().zip(rhs.mask).for_each(|(x, y)| *x = x.$method(y));
                    self
                }
            }

            impl_operations! { ref $op, $method }
        )*
    };
    (ref $op:ident, $method:ident) => {
        impl<'a, const W: usize> $op<Bitmask<W>> for &'a Bitmask<W> {
            type Output = <Bitmask<W> as $op<Bitmask<W>>>::Output;

            #[inline]
            fn $method(self, other: Bitmask<W>) -> <Bitmask<W> as $op<Bitmask<W>>>::Output {
                $op::$method(*self, other)
            }
        }

        impl<const W: usize> $op<&Bitmask<W>> for Bitmask<W> {
            type Output = <Bitmask<W> as $op<Bitmask<W>>>::Output;

            #[inline]
            fn $method(self, other: &Bitmask<W>) -> <Bitmask<W> as $op<Bitmask<W>>>::Output {
                $op::$method(self, *other)
            }
        }

        impl<const W: usize> $op<&Bitmask<W>> for &Bitmask<W> {
            type Output = <Bitmask<W> as $op<Bitmask<W>>>::Output;

            #[inline]
            fn $method(self, other: &Bitmask<W>) -> <Bitmask<W> as $op<Bitmask<W>>>::Output {
                $op::$method(*self, *other)
            }
        }
    }
}

impl_operations! {BitAnd: bitand, BitOr: bitor, BitXor: bitxor}

impl<const W: usize> Debug for Bitmask<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitmask").field("mask", &self.mask).finish()
    }
}

impl<const W: usize> Display for Bitmask<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        printmask(f, self.mask)
    }
}

fn printmask<const N: usize>(f: &mut std::fmt::Formatter<'_>, mask: [u8; N]) -> std::fmt::Result {
    for byte in mask {
        write!(f, "{:08b}", byte)?;
    }
    Ok(())
}

fn main() {
    let mask = Bitmask::from_bytes([1, 128]);

    println!("  1 {:?}", mask);
    println!("  1 {}", mask);

    let mask2 = Bitmask::from_bytes([255, 1]);

    println!("  2 {}", mask2);

    println!("and {}", mask & mask2);
    println!(" or {}", mask | mask2);
    println!("xor {}", mask ^ mask2);
}
