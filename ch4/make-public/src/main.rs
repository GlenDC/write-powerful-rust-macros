use make_public_macro::public;

#[public]
#[derive(Debug)]
struct Example {
    first: String,
    pub second: u32,
}

#[public]
#[derive(Debug)]
struct ExampleUnnamed(String, pub u32);

#[public]
#[derive(Debug)]
enum ExampleEnum {
    First(String),
    Second{ first: String, second: u32 },
}

fn main() {}
