use std::sync::Arc;

fn main() {
    let string = String::from("Hello World");
    let arc_str: Arc<str> = string.into();

    println!("arc_str: {}", arc_str);
}
