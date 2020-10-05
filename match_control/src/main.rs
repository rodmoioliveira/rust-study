// =======================================
// The match Control Flow Operator
// https://doc.rust-lang.org/book/ch06-02-match.html
// =======================================
// Rust has an extremely powerful control flow operator called match that allows you to compare a
// value against a series of patterns and then execute code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards, and many other things;
// Chapter 18 covers all the different kinds of patterns and what they do. The power of match comes
// from the expressiveness of the patterns and the fact that the compiler confirms that all
// possible cases are handled.

// Think of a match expression as being like a coin-sorting machine: coins slide down a track with
// variously sized holes along it, and each coin falls through the first hole it encounters that it
// fits into. In the same way, values go through each pattern in a match, and at the first pattern
// the value “fits,” the value falls into the associated code block to be used during execution.

// Because we just mentioned coins, let’s use them as an example using match! We can write a
// function that can take an unknown United States coin and, in a similar way as the counting
// machine, determine which coin it is and return its value in cents, as shown here in Listing 6-3.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Let’s break down the match in the value_in_cents function. First, we list the match keyword
// followed by an expression, which in this case is the value coin. This seems very similar to an
// expression used with if, but there’s a big difference: with if, the expression needs to return a
// Boolean value, but here, it can be any type. The type of coin in this example is the Coin enum
// that we defined on line 1.

// Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a
// pattern that is the value Coin::Penny and then the => operator that separates the pattern and
// the code to run. The code in this case is just the value 1. Each arm is separated from the next
// with a comma.

// When the match expression executes, it compares the resulting value against the pattern of each
// arm, in order. If a pattern matches the value, the code associated with that pattern is
// executed. If that pattern doesn’t match the value, execution continues to the next arm, much as
// in a coin-sorting machine. We can have as many arms as we need: in Listing 6-3, our match has
// four arms.

// The code associated with each arm is an expression, and the resulting value of the expression in
// the matching arm is the value that gets returned for the entire match expression.

// Curly brackets typically aren’t used if the match arm code is short, as it is in Listing 6-3
// where each arm just returns a value. If you want to run multiple lines of code in a match arm,
// you can use curly brackets. For example, the following code would print “Lucky penny!” every
// time the method was called with a Coin::Penny but would still return the last value of the
// block, 1:

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// =======================================
// Patterns that Bind to Values
// =======================================
// Another useful feature of match arms is that they can bind to the parts of the values that match
// the pattern. This is how we can extract values out of enum variants.

// As an example, let’s change one of our enum variants to hold data inside it. From 1999 through
// 2008, the United States minted quarters with different designs for each of the 50 states on one
// side. No other coins got state designs, so only quarters have this extra value. We can add this
// information to our enum by changing the Quarter variant to include a UsState value stored inside
// it, which we’ve done here in Listing 6-4.

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin_2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Let’s imagine that a friend of ours is trying to collect all 50 state quarters. While we sort
// our loose change by coin type, we’ll also call out the name of the state associated with each
// quarter so if it’s one our friend doesn’t have, they can add it to their collection.

// In the match expression for this code, we add a variable called state to the pattern that
// matches values of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable
// will bind to the value of that quarter’s state. Then we can use state in the code for that arm,
// like so:

fn value_in_cents_2(coin: Coin_2) -> u8 {
    match coin {
        Coin_2::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin_2::Nickel => 5,
        Coin_2::Dime => 10,
        Coin_2::Quarter(state) => {
            println!("The state is {:?}", state);
            25
        }
    }
}

// =======================================
// Matching with Option<T>
// https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
// =======================================

fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;
    println!("{}", value_in_cents(p));
    println!("{}", value_in_cents(n));
    println!("{}", value_in_cents(d));
    println!("{}", value_in_cents(q));
    println!("{}", value_in_cents(Coin::Quarter));

    // If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be
    // Coin::Quarter(UsState::Alaska). When we compare that value with each of the match arms, none
    // of them match until we reach Coin::Quarter(state). At that point, the binding for state will
    // be the value UsState::Alaska. We can then use that binding in the println! expression, thus
    // getting the inner state value out of the Coin enum variant for Quarter.
    let alamaba = Coin_2::Quarter(UsState::Alabama);
    let alaska = Coin_2::Quarter(UsState::Alaska);

    value_in_cents_2(alamaba);
    value_in_cents_2(alaska);
}
