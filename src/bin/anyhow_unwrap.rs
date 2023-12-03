use anyhow::anyhow;

fn get_error() -> anyhow::Result<i32> {
    None.ok_or_else(|| anyhow!("This is the error message!"))
}

fn main() {
    get_error().expect("no error");
}
