use std::io;

fn main() {
    println!("Please enter a temperature below in degrees Celsius");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You typed: {}", guess);
}
