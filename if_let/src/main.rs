// =======================================
// Concise Control Flow with if let
// https://doc.rust-lang.org/book/ch06-03-if-let.html
// =======================================

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn count_coins(count: &mut i32, coin: Coin) {
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => {
            *count = *count + 1;
        }
    }
}

fn main() {
    // The if let syntax lets you combine if and let into a less verbose way to handle values that
    // match one pattern while ignoring the rest. Consider the program in Listing 6-6 that
    // matches on an Option<u8> value but only wants to execute code if the value is 3.

    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("3"),
        _ => (),
    }

    // We want to do something with the Some(3) match but do nothing with any other Some<u8> value
    // or the None value. To satisfy the match expression, we have to add _ => () after processing
    // just one variant, which is a lot of boilerplate code to add.

    // Instead, we could write this in a shorter way using if let. The following code behaves the same
    // as the match in Listing 6-6:

    if let Some(4) = some_u8_value {
        println!("3");
    } else {
        println!("another number!");
    }

    // The syntax if let takes a pattern and an expression separated by an equal sign. It works the
    // same way as a match, where the expression is given to the match and the pattern is its first
    // arm.

    // Using if let means less typing, less indentation, and less boilerplate code. However, you lose
    // the exhaustive checking that match enforces. Choosing between match and if let depends on what
    // you’re doing in your particular situation and whether gaining conciseness is an appropriate
    // trade-off for losing exhaustive checking.

    // In other words, you can think of if let as syntax sugar for a match that runs code when the
    // value matches one pattern and then ignores all other values.

    // We can include an else with an if let. The block of code that goes with the else is the same as
    // the block of code that would go with the _ case in the match expression that is equivalent to
    // the if let and else. Recall the Coin enum definition in Listing 6-4, where the Quarter variant
    // also held a UsState value. If we wanted to count all non-quarter coins we see while also
    // announcing the state of the quarters, we could do that with a match expression like this:

    let mut count = 0;

    count_coins(&mut count, Coin::Nickel);
    count_coins(&mut count, Coin::Quarter(UsState::Alaska));
    count_coins(&mut count, Coin::Quarter(UsState::Alaska));
    count_coins(&mut count, Coin::Quarter(UsState::Alaska));
    count_coins(&mut count, Coin::Quarter(UsState::Alaska));
    count_coins(&mut count, Coin::Penny);
    count_coins(&mut count, Coin::Dime);

    println!(
        "The count is {count} and the count is {count}",
        count = count
    );

    // =======================================
    // Summary
    // =======================================
    // We’ve now covered how to use enums to create custom types that can be one of a set of
    // enumerated values. We’ve shown how the standard library’s Option<T> type helps you use the
    // type system to prevent errors. When enum values have data inside them, you can use match or
    // if let to extract and use those values, depending on how many cases you need to handle.

    // Your Rust programs can now express concepts in your domain using structs and enums. Creating
    // custom types to use in your API ensures type safety: the compiler will make certain your
    // functions get only values of the type each function expects.

    // In order to provide a well-organized API to your users that is straightforward to use and only
    // exposes exactly what your users will need, let’s now turn to Rust’s modules.
}
