use std::{env, error::Error, fs};

pub struct Config {
    // Great! They passed. Now, let’s call the new search_case_insensitive function from the run
    // function. First, we’ll add a configuration option to the Config struct to switch between
    // case-sensitive and case-insensitive search. Adding this field will cause compiler errors
    // because we aren’t initializing this field anywhere yet:
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // =======================================
    // Using the search Function in the run Function
    // =======================================

    // Now that the search function is working and tested, we need to call search from our run
    // function. We need to pass the config.query value and the contents that run reads from the file
    // to the search function. Then run will print each line returned from search:
    let contents = fs::read_to_string(config.filename)?;

    // Note that we added the case_sensitive field that holds a Boolean. Next, we need the run function
    // to check the case_sensitive field’s value and use that to decide whether to call the search
    // function or the search_case_insensitive function, as shown in Listing 12-22. Note this still
    // won’t compile yet.
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// =======================================
// Developing the Library’s Functionality with Test-Driven Development
// =======================================
// Now that we’ve extracted the logic into src/lib.rs and left the argument collecting and error
// handling in src/main.rs, it’s much easier to write tests for the core functionality of our code.
// We can call functions directly with various arguments and check return values without having to
// call our binary from the command line. Feel free to write some tests for the functionality in
// the Config::new and run functions on your own.

// In this section, we’ll add the searching logic to the minigrep program by using the Test-driven
// development (TDD) process. This software development technique follows these steps:

// Write a test that fails and run it to make sure it fails for the reason you expect.  Write or
// modify just enough code to make the new test pass.  Refactor the code you just added or changed
// and make sure the tests continue to pass.  Repeat from step 1!  This process is just one of many
// ways to write software, but TDD can help drive code design as well. Writing the test before you
// write the code that makes the test pass helps to maintain high test coverage throughout the
// process.

// We’ll test drive the implementation of the functionality that will actually do the searching for
// the query string in the file contents and produce a list of lines that match the query. We’ll
// add this functionality in a function called search.

// =======================================
// Writing a Failing Test
// =======================================
// Because we don’t need them anymore, let’s remove the println! statements from src/lib.rs and
// src/main.rs that we used to check the program’s behavior. Then, in src/lib.rs, we’ll add a tests
// module with a test function, as we did in Chapter 11. The test function specifies the behavior
// we want the search function to have: it will take a query and the text to search for the query
// in, and it will return only the lines from the text that contain the query. Listing 12-15 shows
// this test, which won’t compile yet.

// Notice that we need an explicit lifetime 'a defined in the signature of search and used with the
// contents argument and the return value. Recall in Chapter 10 that the lifetime parameters
// specify which argument lifetime is connected to the lifetime of the return value. In this case,
// we indicate that the returned vector should contain string slices that reference slices of the
// argument contents (rather than the argument query).

// In other words, we tell Rust that the data returned by the search function will live as long as
// the data passed into the search function in the contents argument. This is important! The data
// referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes
// we’re making string slices of query rather than contents, it will do its safety checking
// incorrectly.

// If we forget the lifetime annotations and try to compile this function, we’ll get this error:

// $ cargo build
//    Compiling minigrep v0.1.0 (file:///projects/minigrep)
// error[E0106]: missing lifetime specifier
//   --> src/lib.rs:28:51
//    |
// 28 | pub fn search(query: &str, contents: &str) -> Vec<&str> {
//    |                                                   ^ expected lifetime parameter
//    |
//    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0106`.
// error: could not compile `minigrep`.

// To learn more, run the command again with --verbose.

// Rust can’t possibly know which of the two arguments we need, so we need to tell it. Because
// contents is the argument that contains all of our text and we want to return the parts of that
// text that match, we know contents is the argument that should be connected to the return value
// using the lifetime syntax.

// Other programming languages don’t require you to connect arguments to return values in the
// signature. Although this might seem strange, it will get easier over time. You might want to
// compare this example with the “Validating References with Lifetimes” section in Chapter 10.

// =======================================
// Writing Code to Pass the Test
// =======================================
// Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to follow these steps:

// Iterate through each line of the contents.
// Check whether the line contains our query string.
// If it does, add it to the list of values we’re returning.
// If it doesn’t, do nothing.
// Return the list of results that match.
// Let’s work through each step, starting with iterating through lines.

// =======================================
// Iterating Through Lines with the lines Method
// =======================================
// Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines,
// that works as shown in Listing 12-17. Note this won’t compile yet.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // same as let mut results = Vec::new();
    // let mut results = vec![];

    // The lines method returns an iterator. We’ll talk about iterators in depth in Chapter 13, but
    // recall that you saw this way of using an iterator in Listing 3-5, where we used a for loop
    // with an iterator to run some code on each item in a collection.
    // for line in contents.lines() {
    // =======================================
    // Searching Each Line for the Query
    // =======================================

    // Next, we’ll check whether the current line contains our query string. Fortunately, strings have
    // a helpful method named contains that does this for us! Add a call to the contains method in the
    // search function, as shown in Listing 12-18. Note this still won’t compile yet.

    // if line.contains(query) {
    // =======================================
    // Storing Matching Lines
    // =======================================
    // We also need a way to store the lines that contain our query string. For that, we can make a
    // mutable vector before the for loop and call the push method to store a line in the vector. After
    // the for loop, we return the vector, as shown in Listing 12-19.
    // results.push(line);
    // }
    // }

    // At this point, we could consider opportunities for refactoring the implementation of the
    // search function while keeping the tests passing to maintain the same functionality. The code
    // in the search function isn’t too bad, but it doesn’t take advantage of some useful features
    // of iterators. We’ll return to this example in Chapter 13, where we’ll explore iterators in
    // detail, and look at how to improve it.
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// =======================================
// Implementing the search_case_insensitive Function
// =======================================
// The search_case_insensitive function, shown in Listing 12-21, will be almost the same as the
// search function. The only difference is that we’ll lowercase the query and each line so whatever
// the case of the input arguments, they’ll be the same case when we check whether the line
// contains the query.

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // First, we lowercase the query string and store it in a shadowed variable with the same name.
    // Calling to_lowercase on the query is necessary so no matter whether the user’s query is "rust",
    // "RUST", "Rust", or "rUsT", we’ll treat the query as if it were "rust" and be insensitive to the
    // case. While to_lowercase will handle basic Unicode, it won't be 100% accurate. If we were
    // writing a real application, we'd want to do a bit more work here, but this section is about
    // environment variables, not Unicode, so we'll leave it at that here.

    // Note that query is now a String rather than a string slice, because calling to_lowercase creates
    // new data rather than referencing existing data. Say the query is "rUsT", as an example: that
    // string slice doesn’t contain a lowercase u or t for us to use, so we have to allocate a new
    // String containing "rust". When we pass query as an argument to the contains method now, we need
    // to add an ampersand because the signature of contains is defined to take a string slice.

    // Next, we add a call to to_lowercase on each line before we check whether it contains query to
    // lowercase all characters. Now that we’ve converted line and query to lowercase, we’ll find
    // matches no matter what the case of the query is.

    // let query = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }

    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    // This test searches for the string "duct". The text we’re searching is three lines, only one of
    // which contains "duct". We assert that the value returned from the search function contains only
    // the line we expect.

    // We aren’t able to run this test and watch it fail because the test doesn’t even compile: the
    // search function doesn’t exist yet! So now we’ll add just enough code to get the test to compile
    // and run by adding a definition of the search function that always returns an empty vector, as
    // shown in Listing 12-16. Then the test should compile and fail because an empty vector doesn’t
    // match a vector containing the line "safe, fast, productive."

    // Excellent! We’ve built our own mini version of a classic tool and learned a lot about how to
    // structure applications. We’ve also learned a bit about file input and output, lifetimes,
    // testing, and command line parsing.

    // To round out this project, we’ll briefly demonstrate how to work with environment variables and
    // how to print to standard error, both of which are useful when you’re writing command line
    // programs.

    // =======================================
    // Working with Environment Variables
    // https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html
    // =======================================
    // We’ll improve minigrep by adding an extra feature: an option for case-insensitive searching that
    // the user can turn on via an environment variable. We could make this feature a command line
    // option and require that users enter it each time they want it to apply, but instead we’ll use an
    // environment variable. Doing so allows our users to set the environment variable once and have
    // all their searches be case insensitive in that terminal session.

    // =======================================
    // Writing a Failing Test for the Case-Insensitive search Function
    // =======================================
    // We want to add a new search_case_insensitive function that we’ll call when the environment
    // variable is on. We’ll continue to follow the TDD process, so the first step is again to write a
    // failing test. We’ll add a new test for the new search_case_insensitive function and rename our
    // old test from one_result to case_sensitive to clarify the differences between the two tests, as
    // shown in Listing 12-20.

    // Note that we’ve edited the old test’s contents too. We’ve added a new line with the text
    // "Duct tape." using a capital D that shouldn’t match the query "duct" when we’re searching in
    // a case-sensitive manner. Changing the old test in this way helps ensure that we don’t
    // accidentally break the case-sensitive search functionality that we’ve already implemented.
    // This test should pass now and should continue to pass as we work on the case-insensitive
    // search.

    // The new test for the case-insensitive search uses "rUsT" as its query. In the
    // search_case_insensitive function we’re about to add, the query "rUsT" should match the line
    // containing "Rust:" with a capital R and match the line "Trust me." even though both have
    // different casing from the query. This is our failing test, and it will fail to compile because
    // we haven’t yet defined the search_case_insensitive function. Feel free to add a skeleton
    // implementation that always returns an empty vector, similar to the way we did for the search
    // function in Listing 12-16 to see the test compile and fail.
}

// Excellent, we also got lines containing “To”! Our minigrep program can now do case-insensitive
// searching controlled by an environment variable. Now you know how to manage options set using
// either command line arguments or environment variables.

// Some programs allow arguments and environment variables for the same configuration. In those
// cases, the programs decide that one or the other takes precedence. For another exercise on your
// own, try controlling case insensitivity through either a command line argument or an environment
// variable. Decide whether the command line argument or the environment variable should take
// precedence if the program is run with one set to case sensitive and one set to case insensitive.

// The std::env module contains many more useful features for dealing with environment variables:
// check out its documentation to see what is available.

// =======================================
// Writing Error Messages to Standard Error Instead of Standard Output
// https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html
// =======================================
// At the moment, we’re writing all of our output to the terminal using the println! macro. Most
// terminals provide two kinds of output: standard output (stdout) for general information and
// standard error (stderr) for error messages. This distinction enables users to choose to direct
// the successful output of a program to a file but still print error messages to the screen.

// The println! macro is only capable of printing to standard output, so we have to use something
// else to print to standard error.

// =======================================
// Checking Where Errors Are Written
// =======================================
// First, let’s observe how the content printed by minigrep is currently being written to standard
// output, including any error messages we want to write to standard error instead. We’ll do that
// by redirecting the standard output stream to a file while also intentionally causing an error.
// We won’t redirect the standard error stream, so any content sent to standard error will continue
// to display on the screen.

// Command line programs are expected to send error messages to the standard error stream so we can
// still see error messages on the screen even if we redirect the standard output stream to a file.
// Our program is not currently well-behaved: we’re about to see that it saves the error message
// output to a file instead!

// The way to demonstrate this behavior is by running the program with > and the filename,
// output.txt, that we want to redirect the standard output stream to. We won’t pass any arguments,
// which should cause an error:

// $ cargo run > output.txt

// The > syntax tells the shell to write the contents of standard output to output.txt instead of
// the screen. We didn’t see the error message we were expecting printed to the screen, so that
// means it must have ended up in the file. This is what output.txt contains:

// =======================================
// Problem parsing arguments: not enough arguments
// =======================================
// Yup, our error message is being printed to standard output. It’s much more useful for error
// messages like this to be printed to standard error so only data from a successful run ends up in
// the file. We’ll change that.

// =======================================
// Printing Errors to Standard Error
// =======================================
// We’ll use the code in Listing 12-24 to change how error messages are printed. Because of the
// refactoring we did earlier in this chapter, all the code that prints error messages is in one
// function, main. The standard library provides the eprintln! macro that prints to the standard
// error stream, so let’s change the two places we were calling println! to print errors to use
// eprintln! instead.

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     if let Err(e) = minigrep::run(config) {
//         eprintln!("Application error: {}", e);

//         process::exit(1);
//     }
// }

// After changing println! to eprintln!, let’s run the program again in the same way, without any
// arguments and redirecting standard output with >:

// $ cargo run > output.txt
// Problem parsing arguments: not enough arguments

// Now we see the error onscreen and output.txt contains nothing, which is the behavior we expect
// of command line programs.

// Let’s run the program again with arguments that don’t cause an error but still redirect standard
// output to a file, like so:

// $ cargo run to poem.txt > output.txt

// We won’t see any output to the terminal, and output.txt will contain our results:

// Filename: output.txt
// Are you nobody, too?
// How dreary to be somebody!

// This demonstrates that we’re now using standard output for successful output and standard error
// for error output as appropriate.

// =======================================
// Summary
// =======================================
// This chapter recapped some of the major concepts you’ve learned so far and covered how to
// perform common I/O operations in Rust. By using command line arguments, files, environment
// variables, and the eprintln! macro for printing errors, you’re now prepared to write command
// line applications. By using the concepts in previous chapters, your code will be well organized,
// store data effectively in the appropriate data structures, handle errors nicely, and be well
// tested.

// Next, we’ll explore some Rust features that were influenced by functional languages: closures
// and iterators.
