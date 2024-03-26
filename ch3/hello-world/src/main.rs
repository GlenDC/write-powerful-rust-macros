#[macro_use]
extern crate hello_world_macro;

#[derive(Hello)]
enum Pet {
    Cat,
}

#[derive(UpperCaseName)]
struct Example;

fn main() {
    let p = Pet::Cat;
    p.hello_world();
    Pet::testing_testing();

    let e = Example {};
    e.uppercase(); // this prints 'EXAMPLE'
}