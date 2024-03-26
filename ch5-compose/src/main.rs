use compose_macro::{compose, gen_hello_world};

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

struct Greeter;

gen_hello_world!(Greeter);

fn main() {
    let composed = compose!(
        add_one >> add_one >> stringify
    ); 
    println!("{:?}", composed(5));

    println!("{:?}", (Greeter).hello_world());
}
