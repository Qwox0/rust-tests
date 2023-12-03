#![allow(unused)]

use std::{mem::size_of, num::NonZeroUsize, ptr::NonNull};

enum OptionBool {
    Some(bool),
    None,
}

struct TwoBool(bool, bool);

enum OptionTwoBool {
    Some(bool, bool),
    None,
}

fn main() {
    println!("usize:                {}", size_of::<usize>()); // 8
    println!("Option<usize>:        {}", size_of::<Option<usize>>()); // 16
    println!("Option<NonZeroUsize>: {}", size_of::<Option<NonZeroUsize>>()); // 8
    println!("Option<*const ()>:    {}", size_of::<Option<*const ()>>()); // 16
    println!("Option<NonNull<()>>:  {}", size_of::<Option<NonNull<()>>>()); // 8
    println!("bool:                 {}", size_of::<bool>()); // 1
    println!("Option<bool>:         {}", size_of::<Option<bool>>()); // 1
    println!("OptionBool:           {} -> also works for custom types", size_of::<OptionBool>()); // 1
    println!("TwoBool:              {}", size_of::<TwoBool>()); // 2
    println!("OptionTwoBool:        {}", size_of::<OptionTwoBool>()); // 2
}
