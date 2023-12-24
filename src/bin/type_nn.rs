mod const_matrix;

use const_matrix::Matrix;
use std::fmt::Debug;

type Vector<const N: usize> = Box<[f32; N]>;

trait NNItem<const NNIN: usize, const OUT: usize>: Debug + Sized {
    const OUT: usize;

    fn prop(&self, input: Vector<NNIN>) -> Vector<OUT>;

    fn backprop(&mut self) {}

    fn layer<const N: usize>(self, weights: [[f32; OUT]; N]) -> Layer<OUT, N, Self> {
        Layer { item: self, weights: weights.into() }
    }

    fn relu(self) -> ReLU<Self> {
        ReLU { item: self }
    }

    fn build(self) -> NN<NNIN, OUT, Self> {
        NN(self)
    }
}

#[derive(Debug)]
struct Layer<const IN: usize, const OUT: usize, I> {
    item: I,
    weights: Matrix<IN, OUT>,
}

impl<const IN: usize, const OUT: usize, const NNIN: usize, I> NNItem<NNIN, OUT>
    for Layer<IN, OUT, I>
where I: NNItem<NNIN, IN>
{
    const OUT: usize = OUT;

    fn prop(&self, input: Vector<NNIN>) -> Vector<OUT> {
        println!("{:?}", self);
        self.weights.dot(&self.item.prop(input))
    }
}

#[derive(Debug)]
struct ReLU<I> {
    item: I,
}

impl<const N: usize, const NNIN: usize, I> NNItem<NNIN, N> for ReLU<I>
where I: NNItem<NNIN, N>
{
    const OUT: usize = N;

    fn prop(&self, input: Vector<NNIN>) -> Vector<N> {
        println!("{:?}", self);
        self.item.prop(input)
    }
}

#[derive(Debug)]
struct Builder<const X: usize>;

impl<const X: usize> NNItem<X, X> for Builder<X> {
    const OUT: usize = X;

    fn prop(&self, input: Vector<X>) -> Vector<X> {
        input
    }
}

#[derive(Debug)]
struct NN<const IN: usize, const OUT: usize, T>(T);

impl<const IN: usize, const OUT: usize, T: NNItem<IN, OUT>> NN<IN, OUT, T> {
    #[inline]
    fn prop(&self, input: Vector<IN>) -> Vector<OUT> {
        self.0.prop(input)
    }
}

fn main() {
    let nn = Builder
        .layer::<3>([[1.0], [2.0], [4.0]])
        .relu()
        .layer::<2>([[0.0, 0.0, 1.0], [1.0, 1.0, 1.0]])
        .build();

    let out = nn.prop(Box::new([1.0]));
    println!("{:?}", out);

    assert_eq!(out.as_slice(), &[4.0, 7.0])
}

/*
mod const_matrix;

use const_matrix::Matrix;
use std::fmt::Debug;

type Vector<const N: usize> = Box<[f32; N]>;

trait NNItem<const IN: usize, const OUT: usize>: Debug + Sized {
    fn prop(&self, input: Vector<IN>) -> Vector<OUT>;

    fn backprop(&mut self) {}

    fn layer<const LIN: usize>(self, id: usize) -> Layer<LIN, IN, Self> {
        Layer { id, item: self, weights: Default::default() }
    }

    fn relu(self) -> ReLU<Self> {
        ReLU { item: self }
    }

    fn build(self) -> NN<IN, OUT, Self> {
        NN(self)
    }
}

#[derive(Debug)]
struct Layer<const IN: usize, const OUT: usize, I> {
    #[allow(unused)]
    pub id: usize,
    item: I,
    weights: Matrix<IN, OUT>,
}

impl<const IN: usize, const OUT: usize, const NNOUT: usize, I> NNItem<IN, NNOUT>
    for Layer<IN, OUT, I>
where I: NNItem<OUT, NNOUT>
{
    fn prop(&self, input: Vector<IN>) -> Vector<NNOUT> {
        println!("{:?}", self);
        self.item.prop(self.weights.dot(&input))
    }
}

#[derive(Debug)]
struct ReLU<I> {
    item: I,
}

impl<const N: usize, const NNOUT: usize, I> NNItem<N, NNOUT> for ReLU<I>
where I: NNItem<N, NNOUT>
{
    fn prop(&self, input: Vector<N>) -> Vector<NNOUT> {
        println!("{:?}", self);
        self.item.prop(input)
    }
}

#[derive(Debug)]
struct Builder<const X: usize>;

impl<const X: usize> NNItem<X, X> for Builder<X> {
    fn prop(&self, input: Vector<X>) -> Vector<X> {
        input
    }
}

#[derive(Debug)]
struct NN<const IN: usize, const OUT: usize, T>(T);

impl<const IN: usize, const OUT: usize, T: NNItem<IN, OUT>> NN<IN, OUT, T> {
    #[inline]
    fn prop(&self, input: Vector<IN>) -> Vector<OUT> {
        self.0.prop(input)
    }
}

fn main() {
    /*
    let nn = Builder::<1> // a
        .layer::<5>(1.0) // a
        //.relu() // a
        .layer::<2>(2.0) // a
        .build();
    */
    let nn = Layer::<1, 5, _> {
        id: 1,
        item: ReLU {
            item: Layer::<5, 3, _> { id: 3, item: Builder::<3>, weights: Matrix::default() },
        },
        weights: Matrix::default(),
    };

    let out = nn.prop(Box::new([0.0]));
    println!("{:?}", out);

    assert_eq!(out.as_slice(), &[1.0, 99.0, 2.0])
}
 */
