// =======================================
// The Slice Type
// https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type
// =======================================

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    // String Slices
    // A string slice is a reference to part of a String, and it looks like this:
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}!", hello, world);

    // This is similar to taking a reference to the whole String but with the extra [0..5] bit. Rather than a reference to the entire String, it’s a reference to a portion of the String.

    // We can create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. So in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the 7th byte (counting from 1) of s with a length value of 5.

    let s = String::from("oi hello world");
    let slice = &s[3..s.len()];

    // Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. Rust disallows this, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
    // s.clear(); // error!

    println!("{}", slice);
    println!("{}", first_word(&s));

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    // The type of my_string_literal here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // =======================================
    // Other Slices
    // =======================================
    // String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:

    let a = [1, 2, 3, 4, 5];
    // Just as we might want to refer to a part of a string, we might want to refer to part of an array. We’d do so like this:

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    for item in slice {
        println!("{}!", item);
    }
    // This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length. You’ll use this kind of slice for all sorts of other collections. We’ll discuss these collections in detail when we talk about vectors in Chapter 8

    // =======================================
    // Summary
    // =======================================
    // The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

    // Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
}
