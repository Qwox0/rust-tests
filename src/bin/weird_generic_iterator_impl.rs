trait FixIterator {
    fn get_num() -> i32;
}

struct Zeros;

impl FixIterator for Zeros {
    fn get_num() -> i32 {
        0
    }
}

impl<T: FixIterator> Iterator for T {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(T::get_num())
    }
}

/*
impl Iterator for Zeros {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Self::get_num())
    }
}
*/

fn main() {
    let mut zeros = Zeros;
    println!("{:?}", zeros.next());
    println!("{:?}", zeros.next());
    println!("{:?}", zeros.next());
}
