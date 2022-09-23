use debug::debug;

#[derive(Debug)]
struct Struct1 {
    string: String
}

fn main() {
    debug("this will be printed only if the DEBUG environment variable is set");

    let number = 123;
    debug(&format!("{}", number));

    let example_struct = Struct1 { string: String::from("string") };
    debug(&format!("{:?}, field: {}", example_struct, example_struct.string));
}
