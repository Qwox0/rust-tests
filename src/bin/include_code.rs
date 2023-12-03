macro_rules! take_tokens {
    ( $( $tt:tt )*) => {
        concat!($($tt)*)
    };
}

fn main() {
    let text = include!("./include_code_expr.txt");
    println!("Text: {:?}", text);

    // let other = take_tokens!(include!("./include_code_template.html")); // -> doesn't work :(
}
