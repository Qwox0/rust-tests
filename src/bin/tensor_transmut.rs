#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]
#![feature(adt_const_params)]

use core::mem;

pub trait Element: Copy {}

macro_rules! impl_element {
    ($($ty:ty)+) => {
        $(
            impl Element for $ty {}
        )+
    };
}

impl_element! {
    isize i8 i16 i32 i64 i128
    usize u8 u16 u32 u64 u128
    bool
}

pub trait TensorData<X: Element>: Sized {
    const LEN: usize;
}

impl<X: Element> TensorData<X> for X {
    const LEN: usize = 1;
}

impl<X: Element, T: TensorData<X>, const N: usize> TensorData<X> for [T; N] {
    const LEN: usize = T::LEN * N;
}

/// implemented for `Box<[[[[X; A]; B]; ...]; Z]>` (any dimensions)
///
/// `SUB`: sub tensor
/// `LEN`: 1D data length
pub trait Tensor<X: Element, const LEN: usize>:
    Sized + From<Box<Self::DATA>> + Into<Box<Self::DATA>>
{
    type DATA: TensorData<X>;

    fn transmute_from_1d(vec: Box<[X; LEN]>) -> Self {
        // SAFETY: ensure Self::LEN == vec.len();
        Self::from(unsafe { mem::transmute(vec) })
    }

    fn transmute_into_1d(self) -> Box<[X; LEN]> {
        unsafe { mem::transmute(self.into()) }
    }

    fn transmute<T: Tensor<X, LEN>>(self) -> T {
        T::transmute_from_1d(self.transmute_into_1d())
    }

    //const
    fn len(&self) -> usize {
        LEN
    }
}

impl<X: Element, T: TensorData<X>> Tensor<X, { T::LEN }> for Box<T> {
    type DATA = T;
}

pub type Number<X> = Box<X>;
pub type Vector<X, const LEN: usize> = Box<[X; LEN]>;
pub type Matrix<X, const W: usize, const H: usize> = Box<[[X; W]; H]>;
pub type Tensor3<X, const A: usize, const B: usize, const C: usize> = Box<[[[X; A]; B]; C]>;

fn main() {
    let vec = Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    let mat = vec.transmute::<Tensor3<i32, 3, 2, 2>>();
    println!("{:#?}", mat);
    assert_eq!(mat, Box::new([[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]]))
}
