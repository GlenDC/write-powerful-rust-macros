#![allow(unused_imports)]

use panic_to_result_macro::panic_to_result;
use std::convert::Infallible;

#[panic_to_result]
fn create_person_with_empty_panic(name: String, age: u32) -> Result<Person, Infallible> {
    if age > 30 {
        panic!();
    }
    Ok(Person { name, age })
}

fn main() {}
