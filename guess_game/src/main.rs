use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess!");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("readline error");

        println!("your guess is {}", guess.trim());

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
              println!("You Win!");
              break;
            },
        }
    }
}
