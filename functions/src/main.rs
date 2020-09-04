#![allow(unused_variables)]
// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
// =======================================
// Functions
// =======================================
fn main() {
    another_function(5, 10);
    four();
    let x = five();
    println!("{}", x);
    println!("{}", five_2());
    println!("5 + 1 = {}", inc(5));
}

// In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// =======================================
// Functions
// =======================================
// Function bodies are made up of a series of statements optionally ending in an expression. So far, we’ve only covered functions without an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.
// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples.

fn four() {
    let x = 5;

    let y = {
        let x = 3;
        // Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this in mind as you explore function return values and expressions next.
        x + 1
    };

    println!("The value of y is: {}", y);
}

// =======================================
// Functions with Return Values
// =======================================

// Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

// There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32.
fn five() -> i32 {
    5
}

fn five_2() -> i32 {
    return 5;
}

fn inc(x: i32) -> i32 {
    x + 1
}
