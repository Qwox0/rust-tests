#![feature(return_position_impl_trait_in_trait)]

trait I {
    type Out<'a>: Iterator<Item = &'a i32>
    where Self: 'a;

    fn iter2<'a>(&'a self) -> impl Iterator<Item = &'a i32>;
    fn iter3<'a>(&'a self) -> impl Iterator<Item = &'a i32>;
    fn iter4<'a>(&'a self) -> Self::Out<'a>;
}

impl I for Vec<i32> {
    type Out<'a> = core::slice::Iter<'a, i32>;

    fn iter2<'a>(&'a self) -> impl Iterator<Item = &'a i32> {
        self.iter()
    }

    fn iter3<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a i32> {
        self.iter()
    }

    fn iter4<'a>(&'a self) -> Self::Out<'a> {
        self.iter()
    }
}

fn main() {
    let v = vec![1, 2, 3];

    /*
    for x in v.iter2().rev() { // Error: `rev` requires a [`DoubleEndedIterator`]
        println!("{}", x);
    }
    */

    // rust-analyzer problem:
    // type of `x` == <Rev<impl Iterator<Item = &i32>> as IntoIterator>::Item (not a real type)
    for x in v.iter3().rev() {
        println!("{}", x);
    }

    // works as expected
    for x in v.iter4().rev() {
        println!("{}", x);
    }
}
