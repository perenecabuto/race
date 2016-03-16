use std::io::stdin;

fn main() {
    let mut input_line = String::new();

    stdin().read_line(&mut input_line).unwrap();

    print!("{}", input_line);
}
