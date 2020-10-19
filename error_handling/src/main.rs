// =======================================
// Error Handling
// https://doc.rust-lang.org/book/ch09-00-error-handling.html
// =======================================
// Rust’s commitment to reliability extends to error handling. Errors are a fact of life in
// software, so Rust has a number of features for handling situations in which something goes
// wrong. In many cases, Rust requires you to acknowledge the possibility of an error and take some
// action before your code will compile. This requirement makes your program more robust by
// ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your
// code to production!

// Rust groups errors into two major categories: recoverable and unrecoverable errors. For a
// recoverable error, such as a file not found error, it’s reasonable to report the problem to the
// user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to
// access a location beyond the end of an array.

// Most languages don’t distinguish between these two kinds of errors and handle both in the same
// way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type
// Result<T, E> for recoverable errors and the panic! macro that stops execution when the program
// encounters an unrecoverable error. This chapter covers calling panic! first and then talks about
// returning Result<T, E> values. Additionally, we’ll explore considerations when deciding whether
// to try to recover from an error or to stop execution.

fn main() {
    // =======================================
    // Unrecoverable Errors with panic!
    // =======================================
    // Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these
    // cases, Rust has the panic! macro. When the panic! macro executes, your program will print a
    // failure message, unwind and clean up the stack, and then quit. This most commonly occurs
    // when a bug of some kind has been detected and it’s not clear to the programmer how to handle
    // the error.

    // =======================================
    // Unwinding the Stack or Aborting in Response to a Panic

    // By default, when a panic occurs, the program starts unwinding, which means Rust walks back up
    // the stack and cleans up the data from each function it encounters. But this walking back and
    // cleanup is a lot of work. The alternative is to immediately abort, which ends the program
    // without cleaning up. Memory that the program was using will then need to be cleaned up by the
    // operating system. If in your project you need to make the resulting binary as small as possible,
    // you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the
    // appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on
    // panic in release mode, add this:

    // [profile.release]
    // panic = 'abort'
    // =======================================

    // panic!("crash and burn");

    // The call to panic! causes the error message contained in the last two lines. The first line
    // shows our panic message and the place in our source code where the panic occurred:
    // src/main.rs:2:5 indicates that it’s the second line, fifth character of our src/main.rs
    // file.

    // In this case, the line indicated is part of our code, and if we go to that line, we see the
    // panic! macro call. In other cases, the panic! call might be in code that our code calls, and the
    // filename and line number reported by the error message will be someone else’s code where the
    // panic! macro is called, not the line of our code that eventually led to the panic! call. We can
    // use the backtrace of the functions the panic! call came from to figure out the part of our code
    // that is causing the problem. We’ll discuss what a backtrace is in more detail next.

    // =======================================
    // Using a panic! Backtrace
    // =======================================

    // Let’s look at another example to see what it’s like when a panic! call comes from a library
    // because of a bug in our code instead of from our code calling the macro directly. Listing
    // 9-1 has some code that attempts to access an element by index in a vector.

    // Here, we’re attempting to access the 100th element of our vector (which is at index 99
    // because indexing starts at zero), but it has only 3 elements. In this situation, Rust will
    // panic. Using [] is supposed to return an element, but if you pass an invalid index, there’s
    // no element that Rust could return here that would be correct.

    // In C, attempting to read beyond the end of a data structure is undefined behavior. You might get
    // whatever is at the location in memory that would correspond to that element in the data
    // structure, even though the memory doesn’t belong to that structure. This is called a buffer
    // overread and can lead to security vulnerabilities if an attacker is able to manipulate the index
    // in such a way as to read data they shouldn’t be allowed to that is stored after the data
    // structure.

    // To protect your program from this sort of vulnerability, if you try to read an element at an
    // index that doesn’t exist, Rust will stop execution and refuse to continue. Let’s try it and see:

    // The next note line tells us that we can set the RUST_BACKTRACE environment variable to get a
    // backtrace of exactly what happened to cause the error. A backtrace is a list of all the
    // functions that have been called to get to this point. Backtraces in Rust work as they do in
    // other languages: the key to reading the backtrace is to start from the top and read until
    // you see files you wrote. That’s the spot where the problem originated. The lines above the
    // lines mentioning your files are code that your code called; the lines below are code that
    // called your code. These lines might include core Rust code, standard library code, or crates
    // that you’re using. Let’s try getting a backtrace by setting the RUST_BACKTRACE environment
    // variable to any value except 0. Listing 9-2 shows output similar to what you’ll see.

    // let v = vec![1, 2, 3];
    // v[99];

    // That’s a lot of output! The exact output you see might be different depending on your operating
    // system and Rust version. In order to get backtraces with this information, debug symbols must be
    // enabled. Debug symbols are enabled by default when using cargo build or cargo run without the
    // --release flag, as we have here.

    // In the output in Listing 9-2, line 17 of the backtrace points to the line in our project that’s
    // causing the problem: line 4 of src/main.rs. If we don’t want our program to panic, the location
    // pointed to by the first line mentioning a file we wrote is where we should start investigating.
    // In Listing 9-1, where we deliberately wrote code that would panic in order to demonstrate how to
    // use backtraces, the way to fix the panic is to not request an element at index 99 from a vector
    // that only contains 3 items. When your code panics in the future, you’ll need to figure out what
    // action the code is taking with what values to cause the panic and what the code should do
    // instead.

    // We’ll come back to panic! and when we should and should not use panic! to handle error
    // conditions in the “To panic! or Not to panic!” section later in this chapter. Next, we’ll look
    // at how to recover from an error using Result.

    // =======================================
    // Recoverable Errors with Result
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
    // =======================================

    // Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a
    // function fails, it’s for a reason that you can easily interpret and respond to. For example,
    // if you try to open a file and that operation fails because the file doesn’t exist, you might
    // want to create the file instead of terminating the process.

    // Recall from “Handling Potential Failure with the Result Type” in Chapter 2 that the Result enum
    // is defined as having two variants, Ok and Err, as follows:

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // The T and E are generic type parameters: we’ll discuss generics in more detail in Chapter 10.
    // What you need to know right now is that T represents the type of the value that will be returned
    // in a success case within the Ok variant, and E represents the type of the error that will be
    // returned in a failure case within the Err variant. Because Result has these generic type
    // parameters, we can use the Result type and the functions that the standard library has defined
    // on it in many different situations where the successful value and error value we want to return
    // may differ.

    // Let’s call a function that returns a Result value because the function could fail. In Listing
    // 9-3 we try to open a file.

    use std::fs::File;
    let f = File::open("hello.txt");
}
