mod trait_more_bounds;

trait A {
    fn a(&self) -> i32;
}

trait SubA: A {
    fn b(&self) -> i32;
}

impl<T: SubA> A for T {
    fn a(&self) -> i32 {
        -self.b()
    }
}

struct OnlyA;

impl A for OnlyA {
    fn a(&self) -> i32 {
        3
    }
}

struct Both;

impl SubA for Both {
    fn b(&self) -> i32 {
        10
    }
}

fn main() {
    let a = OnlyA;
    let both = Both;

    println!("a.a(): {}", a.a());
    // println!("a.b(): {}", a.b()); // Error
    println!("both.a(): {}", both.a());
    println!("both.b(): {}", both.b());
}
