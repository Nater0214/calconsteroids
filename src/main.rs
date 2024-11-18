use std::io;

use calconsteroids::parse::{parse_latex, parse_pairs};

/// The entrypoint to this program
fn main() {
    // Get an expression from the user
    let mut input = String::new();
    println!("Enter an expression: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    // Parse the expression
    let pairs = parse_latex(input).expect("Bad expression");
    dbg!(&pairs);
    let expression = parse_pairs(pairs);

    // Print the simplified expression
    println!("{}", expression.simplified());
}
