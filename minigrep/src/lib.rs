use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
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

    for line in search(&config.query, &contents) {
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
    let mut results = vec![];
    // The lines method returns an iterator. We’ll talk about iterators in depth in Chapter 13, but
    // recall that you saw this way of using an iterator in Listing 3-5, where we used a for loop
    // with an iterator to run some code on each item in a collection.
    for line in contents.lines() {
        // =======================================
        // Searching Each Line for the Query
        // =======================================

        // Next, we’ll check whether the current line contains our query string. Fortunately, strings have
        // a helpful method named contains that does this for us! Add a call to the contains method in the
        // search function, as shown in Listing 12-18. Note this still won’t compile yet.

        if line.contains(query) {
            // =======================================
            // Storing Matching Lines
            // =======================================
            // We also need a way to store the lines that contain our query string. For that, we can make a
            // mutable vector before the for loop and call the push method to store a line in the vector. After
            // the for loop, we return the vector, as shown in Listing 12-19.
            results.push(line);
        }
    }

    // At this point, we could consider opportunities for refactoring the implementation of the
    // search function while keeping the tests passing to maintain the same functionality. The code
    // in the search function isn’t too bad, but it doesn’t take advantage of some useful features
    // of iterators. We’ll return to this example in Chapter 13, where we’ll explore iterators in
    // detail, and look at how to improve it.
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
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
