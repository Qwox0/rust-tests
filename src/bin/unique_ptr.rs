#![feature(ptr_internals)]
#![feature(vec_into_raw_parts)]

use std::{ptr::Unique, vec};

fn get_unique() -> Unique<i32> {
    let vec = vec![1, 3, 5];
    let ptr = vec.into_raw_parts().0;
    Unique::new(ptr).unwrap()
}

fn main() {
    let mut ptr = get_unique();

    println!("{:?}", ptr);

    print(ptr);

    let r = unsafe { ptr.as_ref() };

    *unsafe { ptr.as_mut() } = 10;

    print(ptr);

    // dbg!(r); // error
}

fn print(ptr: Unique<i32>) {
    print!("[");
    for i in 0..3 {
        print!(" {:?},", unsafe { *ptr.as_ptr().add(i) });
    }
    println!(" ]");
}
