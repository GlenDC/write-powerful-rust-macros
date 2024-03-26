use private_macro::private;

private!(
    struct Example {
        pub string_value: String,
        number_value: i32,
    }
);

fn main() {
    let e = Example {
        string_value: "value".to_string(),
        number_value: 2,
    };

    println!("{}", e.get_string_value());
    println!("{}", e.get_number_value());
}