use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Inner {
    arr: [i32; 10],
}

impl Index<usize> for Inner {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

mod inherit {
    use crate::Inner;
    use std::ops::Deref;

    pub struct Outer {
        pub inner: Inner,
    }

    impl Deref for Outer {
        type Target = Inner;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}

mod overwrite {
    use crate::Inner;
    use std::ops::{Deref, Index};

    pub struct Outer {
        pub inner: Inner,
    }

    impl Deref for Outer {
        type Target = Inner;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl Index<usize> for Outer {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.arr[9 - index]
        }
    }
}

fn main() {
    let inner = Inner { arr: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] };

    println!("          inner[1] = {:?}", inner[1]); // 1

    let outer = inherit::Outer { inner };

    println!("  inherit outer[1] = {:?}", outer[1]); // 1

    let outer = overwrite::Outer { inner };

    println!("overwrite outer[1] = {:?}", outer[1]); // 8
}
