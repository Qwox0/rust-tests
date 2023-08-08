use std::{fmt::Debug, marker::PhantomData};

pub struct Wrapper<T>(pub T);

trait Generic<T> {}
trait Type {
    type Item;
}

struct Impl<T>(pub T);
impl<T> Generic<T> for Impl<T> {}

trait Test<T>: Sized {
    //fn generic_w_impl<'a>(batch: impl Generic<&'a Wrapper<T>>);
    fn generic_w_impl(batch: impl for<'a> Generic<&'a Wrapper<T>>);
    //fn generic_t_impl<'a>(batch: impl Generic<&'a T>);
    fn generic_t_impl(batch: impl for<'a> Generic<&'a T>);

    //fn generic_w<'a, G: Generic<&'a Wrapper<T>>>(batch: G);
    fn generic_w<G: for<'a> Generic<&'a Wrapper<T>>>(batch: G);
    //fn generic_t<'a, G: Generic<&'a T>>(batch: G);
    fn generic_t<G: for<'a> Generic<&'a T>>(batch: G);
    fn generic_t2<G>(batch: G)
    where for<'a> G: Generic<&'a T>;

    fn type_w_impl<'a>(batch: impl Type<Item = &'a Wrapper<T>>)
    where T: 'a;
    fn type_t_impl<'a>(batch: impl Type<Item = &'a T>)
    where T: 'a;

    //fn type_w<'a, U: Type<Item = &'a Wrapper<T>>>(batch: T);
    //fn type_t<'a, U: Type<Item = &'a T>>(batch: T);

    fn impl_w<'a>(batch: Impl<&'a Wrapper<T>>);
    fn impl_t<'a>(batch: Impl<&'a T>);

    fn w<'a>(batch: &'a Wrapper<T>);
    fn t<'a>(batch: &'a T);
}

trait WithIter<'a> {
    fn with_iter<T: 'a>(iter: impl Iterator<Item = &'a T>) {
        todo!()
    }
}

fn main() {
    let a = Impl::<isize>(10);
}
