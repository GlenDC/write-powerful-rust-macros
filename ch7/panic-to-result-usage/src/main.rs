#![allow(dead_code)]

use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

#[panic_to_result]
fn create_person(name: String, age: u32) -> Person {
    if age > 30 {
        panic!("I hope I die before I get old");
    }
    Person { name, age }
}

fn main() {
    let person = create_person("Sam".to_string(), 22).unwrap();
    println!("{:?}", person);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam".to_string());
        assert_eq!(actual.age, 22);
    }

    #[test]
    fn should_err_on_invalid_age() {
        let actual = create_person("S".to_string(), 33);

        assert_eq!(
            actual.expect_err("this should be an err"),
            "I hope I die before I get old".to_string()
        );
    }
}
