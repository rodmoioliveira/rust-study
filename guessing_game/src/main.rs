use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input a guess:");

    let mut guess = String::new();

    io::stdin()
        // The next part of the code, .read_line(&mut guess), calls the read_line method on the standard input handle to get input from the user. We’re also passing one argument to read_line: &mut guess.
        // The job of read_line is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);
}
