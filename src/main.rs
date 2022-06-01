use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn capacity(n: u32) {
    let range: u128 = 2_u128.pow(n - 1);
    println!("i{} can range from -{} to {}", n, range, range - 1);
    println!("u{} can range from 0 to {}\n", n, (range - 1) * 2 + 1);
}

fn game() {
    println!("Enter a number between 1 and 100 : ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input!");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please enter an unsigned integer between 1 an 100!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The number is {}.\n", secret_number);
}

fn main() {
    // game();

    capacity(8);
    capacity(16);
    capacity(32);
    capacity(64);
    capacity(128);
}
