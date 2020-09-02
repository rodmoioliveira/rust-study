fn main() {
    // =======================================
    // Differences Between Variables and Constants
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
    // =======================================

    // Being unable to change the value of a variable might have reminded you of another programming concept that most other languages have: constants. Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

    // There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // Here’s an example of a constant declaration where the constant’s name is MAX_POINTS and its value is set to 100,000. (Rust’s naming convention for constants is to use all uppercase with underscores between words, and underscores can be inserted in numeric literals to improve readability):
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // =======================================
    // Shadowing
    // =======================================
    // As you saw in the guessing game tutorial in the “Comparing the Guess to the Secret Number” section in Chapter 2, you can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword as follows:
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, but we really want to store that input as a number
    let spaces = "                    ";
    let spaces = spaces.len();
    println!("Spaces is: {}", &spaces);

    // =======================================
    // Data Types
    // https://doc.rust-lang.org/book/ch03-02-data-types.html
    // =======================================
}
