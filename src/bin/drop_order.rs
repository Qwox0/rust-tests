use std::fmt::Display;

struct PrintOnDrop<T: Display>(T);

impl<T: Display> Drop for PrintOnDrop<T> {
    fn drop(&mut self) {
        println!("{}", self.0)
    }
}

fn main() {
    PrintOnDrop(1);
    PrintOnDrop(2);
    println!("{}", 3);
    PrintOnDrop("Hello World");
    PrintOnDrop(4);
}
