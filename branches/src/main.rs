// https://doc.rust-lang.org/book/ch03-05-control-flow.html

// =======================================
// Control Flow
// =======================================
fn main() {
    let number = 6;

    // It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The error indicates that Rust expected a bool but got an integer. Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition. If we want the if code block to run only when a number is not equal to 0, for example, we can change the if expression to the following:
    if number != 0 {
        println!("number was something other than zero");
    }

    // You can have multiple conditions by combining if and else in an else if expression. For example:
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called match for these cases.
    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    // Because if is an expression, we can use it on the right side of a let statement.
    let value = if number > 5 {
        "greater than 5"
    } else {
        "less than five"
    };

    println!("The value of number is {}", value);

    // =======================================
    // Repetition with Loops
    // =======================================
    // It’s often useful to execute a block of code more than once. For this task, Rust provides several loops. A loop runs through the code inside the loop body to the end and then starts immediately back at the beginning. To experiment with loops, let’s make a new project called loops.

    // Rust has three kinds of loops: loop, while, and for. Let’s try each one.
    let mut counter = 0;

    let result = loop {
        println!("The counter is {}", counter);
        counter += 1;

        if counter == 10 {
            // Before the loop, we declare a variable named counter and initialize it to 0. Then we declare a variable named result to hold the value returned from the loop. On every iteration of the loop, we add 1 to the counter variable, and then check whether the counter is equal to 10. When it is, we use the break keyword with the value counter * 2. After the loop, we use a semicolon to end the statement that assigns the value to result. Finally, we print the value in result, which in this case is 20.
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // =======================================
    // Conditional Loops with while
    // =======================================
    let mut number = 10;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // =======================================
    // Looping Through a Collection with for
    // =======================================
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    for number in 1..40 {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

