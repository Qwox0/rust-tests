use std::{fmt::Debug, marker::PhantomData};

struct FnBoundTestImpl<T>(PhantomData<T>);
trait FnBoundTest<T> {
    fn print(&self, t: T)
    where T: Debug {
        println!("{:?}", t);
    }

    fn copy(&self, t: T) -> T
    where T: Copy {
        t.clone()
    }
}
impl<T> FnBoundTest<T> for FnBoundTestImpl<T> {}

fn main() {
    let fn_bound_test_usize = FnBoundTestImpl::<usize>(PhantomData);
    let fn_bound_test_string = FnBoundTestImpl::<String>(PhantomData);

    fn_bound_test_usize.print(10);
    dbg!(fn_bound_test_usize.copy(10));

    let string1 = "hello world".to_string();
    let string2 = "copy me".to_string();
    fn_bound_test_string.print(string1);
    // dbg!(fn_bound_test_string.copy(string2));
}
