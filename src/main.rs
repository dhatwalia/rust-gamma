use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
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

    println!("The number is {}.", secret_number)
}
