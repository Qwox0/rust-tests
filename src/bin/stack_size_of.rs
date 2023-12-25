use std::mem::size_of;

fn main() {
    println!("  *const (): {:?}", size_of::<*const ()>()); // 8
    println!("    *mut (): {:?}", size_of::<*mut ()>()); // 8

    println!("  Box slice: {:?}", size_of::<Box<[f64]>>()); // 16
    println!("  Box array: {:?}", size_of::<Box<[f64; 10]>>()); // 8

    println!("      f64: {:?}", size_of::<f64>()); // 8
    println!(" [f64; 1]: {:?}", size_of::<[f64; 1]>()); // 8
    println!("[f64; 10]: {:?}", size_of::<[f64; 10]>()); // 80
}
