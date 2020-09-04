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

    // =======================================
    // Scalar Types
    // =======================================

    // A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    // =======================================
    // Integer Types
    // =======================================
    let decimal = 98_222;
    let hex = 0xffffff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!(
        "Decimal is {}\nhex is {}\noctal is {}\nbinary is {}\nbyte is {}",
        decimal, hex, octal, binary, byte
    );

    // =======================================
    // Floating-Point Types
    // =======================================
    // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // =======================================
    // Numeric Operations
    // =======================================

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum is {}\ndifference is {}\nproduct is {}\nquocient is {}\nremainder is {}",
        sum, difference, product, quotient, remainder
    );

    // =======================================
    // The Boolean Type
    // =======================================
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // =======================================
    // The Character Type
    // =======================================

    // So far we’ve worked only with numbers, but Rust supports letters too. Rust’s char type is the language’s most primitive alphabetic type, and the following code shows one way to use it. (Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.)
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // =======================================
    // Compound Types
    // =======================================
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // =======================================
    // The Tuple Type
    // =======================================

    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    // We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // In addition to destructuring through pattern matching, we can access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);

    // =======================================
    // The Array Type
    // =======================================
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

    // In Rust, the values going into an array are written as a comma-separated list inside square brackets:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[4]);

    // Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a vector, you should probably use a vector. Chapter 8 discusses vectors in more detail.
    // https://www.geeksforgeeks.org/memory-layout-of-c-program/
    // https://www.geeksforgeeks.org/stack-vs-heap-memory-allocation/

    // An example of when you might want to use an array rather than a vector is in a program that needs to know the names of the months of the year. It’s very unlikely that such a program will need to add or remove months, so you can use an array because you know it will always contain 12 elements:

    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // Writing an array’s type this way looks similar to an alternative syntax for initializing an array: if you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

    let _a = [3; 5];
}
