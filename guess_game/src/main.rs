use std::io;

fn main() {
    println!("Input your guess!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("readline error");

    println!("your guess is {guess}");
}
