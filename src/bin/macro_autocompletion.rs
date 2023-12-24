#![allow(unused)]

#[derive(Default, Debug)]
struct Foo {
    bar: i32,
    baz: i32,
}

/// works
macro_rules! create_foo1 {
    ($field:ident) => {{ Foo { $field: 10, baz: 10 } }};
}

/// works
macro_rules! create_foo2 {
    ($field:ident : $val:expr) => {{ Foo { $field: $val, baz: $val } }};
}

/// works
macro_rules! create_foo3 {
    ( $( $field:ident )* ) => {{
        Foo { $( $field: 10),* }
    }};
}

/// works
macro_rules! create_foo4 {
    ( $( $field:ident ),* ) => {{
        Foo { $( $field: 10),* }
    }};
}

/// doesn't work
/// requires to pretype the ','
macro_rules! create_foo5 {
    ( $( $field:ident , )* ) => {{
        Foo { $( $field: 10 ),* }
    }};
}

/// doesn't work
macro_rules! create_foo6 {
    ( $( $field:ident : $val:expr ),* $(,)? ) => {{
        Foo { $( $field: $val ),* }
    }};
}

macro_rules! create_foo_fix {
    ( $( $tt:tt )* ) => {{
        Foo { $($tt)* ..Default::default() }
    }};
}

struct FooBuilder {}
impl FooBuilder {
    fn bar(self, val: i32) -> Self {
        self
    }

    fn baz(self, val: i32) -> Self {
        self
    }
}


macro_rules! create_foo_fix2 {
    ( $( $var:ident : $val:expr )* ) => {{
        FooBuilder
            $(.$var($val))*
    }};
}

fn main() {
    const SOME_NUM: i32 = 5;

    let foo = create_foo_fix! {
        baz: SOME_NUM,
    };

    println!("{:?}", foo);

    let foo = create_foo_fix2! {
        bar: 10, baz: 10
    };
}
