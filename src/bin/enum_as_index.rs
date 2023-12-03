enum MyIndex {
    A,
    B,
    Z,
}

fn main() {
    let mut arr = vec![0, 0, 0, 7, 23];

    println!("arr: {:?}", arr);

    arr[MyIndex::A as usize] = 'A' as u8;
    arr[MyIndex::B as usize] = 'B' as u8;
    arr[MyIndex::Z as usize] = 'Z' as u8;

    println!("arr: {:?}", arr);
}
