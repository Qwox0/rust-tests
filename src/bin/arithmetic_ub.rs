#![feature(unchecked_neg)]
#![feature(unchecked_math)]

unsafe fn run_ub() {
    println!("i8::MIN.unchecked_neg() == {}", i8::MIN.unchecked_neg());
    println!(" i8::MIN.wrapping_neg() == {}", i8::MIN.wrapping_neg());
    println!("u8::MAX.unchecked_add(1) == {}", u8::MAX.unchecked_add(1));
    println!(" u8::MAX.wrapping_add(1) == {}", u8::MAX.wrapping_add(1));
}

fn main() {
    unsafe { run_ub() }
}
