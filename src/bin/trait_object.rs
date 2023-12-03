trait MyTrait {
    fn func()
    where Self: Sized {
    }
}

fn box_dyn_(obj: Box<dyn MyTrait>) {}
fn ref_dyn_(obj: &dyn MyTrait) {}

fn main() {
    println!("Hello, world!");
}
