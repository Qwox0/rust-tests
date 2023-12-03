use std::mem::size_of;

fn main() {
    println!("*const (): {:?}", size_of::<*const ()>()); // 8
    println!("  *mut (): {:?}", size_of::<*mut ()>()); // 8

    println!("Box slice: {:?}", size_of::<Box<[f64]>>()); // 16
    println!("Box array: {:?}", size_of::<Box<[f64; 10]>>()); // 8
}
