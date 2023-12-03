#![feature(arbitrary_self_types)]

use std::ops::{Add, Deref};

struct Wrapper<T>(T);

impl Wrapper<&i32> {
    fn get(&self) -> &i32 {
        self.0
    }
}

impl Wrapper<&mut i32> {
    fn get(&self) -> &i32 {
        self.0
    }

    fn inc(&mut self) {
        *self.0 += 1;
    }
}

trait WrapRef {
    fn wrap(&self) -> Wrapper<&Self>;
}

trait WrapMut {
    fn wrap(&mut self) -> Wrapper<&mut Self>;
}

impl WrapRef for i32 {
    fn wrap(&self) -> Wrapper<&Self> {
        Wrapper(self)
    }
}

impl WrapMut for i32 {
    fn wrap(&mut self) -> Wrapper<&mut Self> {
        Wrapper(self)
    }
}

fn wrap<T>(v: T) -> Wrapper<T> {
    Wrapper(v)
}

// ======================

#[derive(Debug)]
pub struct NN {
    val: u16,
}

impl AsRef<NN> for NN {
    fn as_ref(&self) -> &NN {
        self
    }
}

impl AsMut<NN> for NN {
    fn as_mut(&mut self) -> &mut NN {
        self
    }
}

pub struct W<R: AsRef<NN>>(R);

impl<R: AsRef<NN>> W<R> {
    pub fn get(&self) -> u16 {
        self.0.as_ref().val
    }
}

impl<R: AsRef<NN> + AsMut<NN>> W<R> {
    pub fn inc(&mut self) {
        self.0.as_mut().val += 1;
    }
}

// =============================

impl NN {
    pub fn propagate() {

    }

    pub fn backpropagate() {

    }

}

fn main() {
    let mut value = 10;

    let nn = NN { val: 123 };

    let mut w = W(nn);

    //let mut w = value.wrap2(3);
    //let mut w = wrap(&mut value);

    println!("{:?}", w.get());
    w.inc();
    println!("{:?}", w.get());

    println!("{:?}", nn);
}

/*
trait RefFamily<'a> {
    type Ref<T: 'a>: Deref<Target = T>;
}

struct ShrRef;
impl<'a> RefFamily<'a> for ShrRef {
    type Ref<T: 'a> = &'a T;
}

struct MutRef;
impl<'a> RefFamily<'a> for MutRef {
    type Ref<T: 'a> = &'a mut T;
}

struct Foo(i32);
impl Foo {
    fn accessor<'a, T: RefFamily<'a>>(self: T::Ref<Self>) -> T::Ref<i32> {
        self
    }
}
*/
