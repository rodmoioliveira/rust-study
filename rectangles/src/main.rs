// =======================================
// An Example Program Using Structs
// https://doc.rust-lang.org/book/ch05-02-example-structs.html
// =======================================
// To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. We’ll start with single variables, and then refactor the program until we’re using structs instead.
// Let’s make a new binary project with Cargo called rectangles that will take the width and height of a rectangle specified in pixels and calculate the area of the rectangle. Listing 5-8 shows a short program with one way of doing exactly that in our project’s src/main.rs.

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    // Adding Useful Functionality with Derived Traits
    // It’d be nice to be able to print an instance of Rectangle while we’re debugging our program and see the values for all its fields.
    // The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption. The primitive types we’ve seen so far implement Display by default, because there’s only one way you’d want to show a 1 or any other primitive type to a user. But with structs, the way println! should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want, and structs don’t have a provided implementation of Display.
    // If we continue reading the errors, we’ll find this helpful note:
    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

    // Let’s try it! The println! macro call will now look like println!("rect1 is {:?}", rect1);. Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

    // Compile the code with this change. Drat! We still get an error:
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
    // Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the annotation #[derive(Debug)] just before the struct definition,
    // Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string.
    // Rust has provided a number of traits for us to use with the derive annotation that can add useful behavior to our custom types. Those traits and their behaviors are listed in Appendix C. We’ll cover how to implement these traits with custom behavior as well as how to create your own traits in Chapter 10.
    // Our area function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.
}

// The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters. The parameters are related, but that’s not expressed anywhere in our program. It would be more readable and more manageable to group width and height together. We’ve already discussed one way we might do that in “The Tuple Type” section of Chapter 3: by using tuples.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

// =======================================
// Refactoring with Structs: Adding More Meaning
// =======================================
// We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a data type with a name for the whole as well as names for the parts, as shown in Listing 5-10.

// Here we’ve defined a struct and named it Rectangle. Inside the curly brackets, we defined the fields as width and height, both of which have type u32. Then in main, we created a particular instance of Rectangle that has a width of 30 and a height of 50.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.

// The area function accesses the width and height fields of the Rectangle instance. Our function signature for area now says exactly what we mean: calculate the area of Rectangle, using its width and height fields. This conveys that the width and height are related to each other, and it gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity.
fn area_2(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
