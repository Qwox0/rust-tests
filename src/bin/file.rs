use std::{fs::File, io::Write};

fn main() {
    let mut file = File::create("./file").unwrap();

    println!("{:?}", file);
    file.write_all(b"before\n").unwrap();
    loop {}
    file.write_all(b"after\n").unwrap();
    println!("{:?}", file);
}
