//! <https://www.youtube.com/watch?v=wU8hQvU8aKM>

trait Speak {
    fn speak(&self);
}

struct Animal {
    data: i32,
}

impl Speak for Animal {
    fn speak(&self) {
        println!("woof!")
    }
}

fn listen_to(obj: &dyn Speak) {
    print!("listen to: ");
    obj.speak();
}

fn main() {
    let a = Animal { data: 0 };
    a.speak(); // -> woof!

    let ref_dyn = &a as &dyn Speak;
    ref_dyn.speak(); // -> woof!

    listen_to(&a); // -> listen to: woof!
}
