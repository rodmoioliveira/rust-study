use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");
    println!("Please input a guess:");

    // The next part of the code, .read_line(&mut guess), calls the read_line method on the standard input handle to get input from the user. We’re also passing one argument to read_line: &mut guess.
    // The job of read_line is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.
    // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is: {}", secret_number);
}
