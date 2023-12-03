//! <https://rust-lang.github.io/rfcs/1210-impl-specialization.html>

#![feature(specialization)]

use std::ops::Add;

pub trait MyTrait {
    fn my_add_assign(&mut self, rhs: Self) -> &str;
}

impl<T: Add<Output = T> + Clone> MyTrait for T {
    default fn my_add_assign(&mut self, rhs: Self) -> &str {
        let tmp = self.clone() + rhs;
        *self = tmp;
        "default"
    }
}

impl MyTrait for i32 {
    fn my_add_assign(&mut self, rhs: Self) -> &str {
        *self += rhs;
        "specialization"
    }
}

fn main() {
    let mut num: usize = 5;
    let default_res = num.my_add_assign(10);
    println!("{}", default_res);

    let mut num: i32 = 5;
    let special_res = num.my_add_assign(10);
    println!("{}", special_res);
}
