#![feature(iter_array_chunks)]

fn main() {
    let iterator = 1..;
    let vec: Vec<_> = iterator.array_chunks().take(5).map(|[a, b]| (a, b)).collect();

    println!("{:?}", vec);
}
