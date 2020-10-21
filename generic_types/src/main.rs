// =======================================
// Generic Types, Traits, and Lifetimes
// https://doc.rust-lang.org/book/ch10-00-generics.html
// =======================================
// Every programming language has tools for effectively handling the duplication of concepts. In
// Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other
// properties. When we’re writing code, we can express the behavior of generics or how they relate
// to other generics without knowing what will be in their place when compiling and running the
// code.

// Similar to the way a function takes parameters with unknown values to run the same code on
// multiple concrete values, functions can take parameters of some generic type instead of a
// concrete type, like i32 or String. In fact, we’ve already used generics in Chapter 6 with
// Option<T>, Chapter 8 with Vec<T> and HashMap<K, V>, and Chapter 9 with Result<T, E>. In this
// chapter, you’ll explore how to define your own types, functions, and methods with generics!

// First, we’ll review how to extract a function to reduce code duplication. Next, we’ll use the
// same technique to make a generic function from two functions that differ only in the types of
// their parameters. We’ll also explain how to use generic types in struct and enum definitions.

// Then you’ll learn how to use traits to define behavior in a generic way. You can combine traits
// with generic types to constrain a generic type to only those types that have a particular
// behavior, as opposed to just any type.

// Finally, we’ll discuss lifetimes, a variety of generics that give the compiler information about
// how references relate to each other. Lifetimes allow us to borrow values in many situations
// while still enabling the compiler to check that the references are valid.

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    // =======================================
    // Removing Duplication by Extracting a Function
    // =======================================
    // Before diving into generics syntax, let’s first look at how to remove duplication that doesn’t
    // involve generic types by extracting a function. Then we’ll apply this technique to extract a
    // generic function! In the same way that you recognize duplicated code to extract into a function,
    // you’ll start to recognize duplicated code that can use generics.

    // Consider a short program that finds the largest number in a list, as shown in Listing 10-1.

    let number_list = vec![34, 50, 25, 100, 65, 1000];

    // The largest function has a parameter called list, which represents any concrete slice of i32
    // values that we might pass into the function. As a result, when we call the function, the
    // code runs on the specific values that we pass in.

    // In sum, here are the steps we took to change the code from Listing 10-2 to Listing 10-3:

    // Identify duplicate code.  Extract the duplicate code into the body of the function and specify
    // the inputs and return values of that code in the function signature.  Update the two instances
    // of duplicated code to call the function instead.  Next, we’ll use these same steps with generics
    // to reduce code duplication in different ways. In the same way that the function body can operate
    // on an abstract list instead of specific values, generics allow code to operate on abstract
    // types.

    // For example, say we had two functions: one that finds the largest item in a slice of i32 values
    // and one that finds the largest item in a slice of char values. How would we eliminate that
    // duplication? Let’s find out!

    println!("The largest number is {}", largest(&number_list));

    // =======================================
    // Generic Data Types
    // https://doc.rust-lang.org/book/ch10-01-syntax.html
    // =======================================
}
