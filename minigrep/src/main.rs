// =======================================
// An I/O Project: Building a Command Line Program
// https://doc.rust-lang.org/book/ch12-00-an-io-project.html
// =======================================
// This chapter is a recap of the many skills you’ve learned so far and an exploration of a few
// more standard library features. We’ll build a command line tool that interacts with file and
// command line input/output to practice some of the Rust concepts you now have under your belt.

// Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language
// for creating command line tools, so for our project, we’ll make our own version of the classic
// command line tool grep (globally search a regular expression and print). In the simplest use
// case, grep searches a specified file for a specified string. To do so, grep takes as its
// arguments a filename and a string. Then it reads the file, finds lines in that file that contain
// the string argument, and prints those lines.

// Along the way, we’ll show how to make our command line tool use features of the terminal that
// many command line tools use. We’ll read the value of an environment variable to allow the user
// to configure the behavior of our tool. We’ll also print error messages to the standard error
// console stream (stderr) instead of standard output (stdout), so, for example, the user can
// redirect successful output to a file while still seeing error messages onscreen.

// One Rust community member, Andrew Gallant, has already created a fully featured, very fast
// version of grep, called ripgrep. By comparison, our version of grep will be fairly simple, but
// this chapter will give you some of the background knowledge you need to understand a real-world
// project such as ripgrep.

// Our grep project will combine a number of concepts you’ve learned so far:

// Organizing code (using what you learned about modules in Chapter 7) Using vectors and strings
// (collections, Chapter 8) Handling errors (Chapter 9) Using traits and lifetimes where
// appropriate (Chapter 10) Writing tests (Chapter 11) We’ll also briefly introduce closures,
// iterators, and trait objects, which Chapters 13 and 17 will cover in detail.

// =======================================
// Accepting Command Line Arguments
// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
// =======================================

// The first task is to make minigrep accept its two command line arguments: the filename and a
// string to search for. That is, we want to be able to run our program with cargo run, a string to
// search for, and a path to a file to search in, like so:

// $ cargo run searchstring example-filename.txt

// =======================================
//Reading the Argument Values
// =======================================

// extern crate minigrep;
use minigrep;
use std::{env, process};

fn main() {
    // To enable minigrep to read the values of command line arguments we pass to it, we’ll need a
    // function provided in Rust’s standard library, which is std::env::args. This function returns
    // an iterator of the command line arguments that were given to minigrep. We’ll cover iterators
    // fully in Chapter 13. For now, you only need to know two details about iterators: iterators
    // produce a series of values, and we can call the collect method on an iterator to turn it
    // into a collection, such as a vector, containing all the elements the iterator produces.

    // Use the code in Listing 12-1 to allow your minigrep program to read any command line arguments
    // passed to it and then collect the values into a vector.

    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // First, we bring the std::env module into scope with a use statement so we can use its args
    // function. Notice that the std::env::args function is nested in two levels of modules. As we
    // discussed in Chapter 7, in cases where the desired function is nested in more than one
    // module, it’s conventional to bring the parent module into scope rather than the function. By
    // doing so, we can easily use other functions from std::env. It’s also less ambiguous than
    // adding use std::env::args and then calling the function with just args, because args might
    // easily be mistaken for a function that’s defined in the current module.

    // The args Function and Invalid Unicode
    // Note that std::env::args will panic if any argument contains invalid Unicode. If your program
    // needs to accept arguments containing invalid Unicode, use std::env::args_os instead. That
    // function returns an iterator that produces OsString values instead of String values. We’ve
    // chosen to use std::env::args here for simplicity, because OsString values differ per platform
    // and are more complex to work with than String values.

    // On the first line of main, we call env::args, and we immediately use collect to turn the
    // iterator into a vector containing all the values produced by the iterator. We can use the
    // collect function to create many kinds of collections, so we explicitly annotate the type of
    // args to specify that we want a vector of strings. Although we very rarely need to annotate
    // types in Rust, collect is one function you do often need to annotate because Rust isn’t able
    // to infer the kind of collection you want.

    // Notice that the first value in the vector is "target/debug/minigrep", which is the name of
    // our binary. This matches the behavior of the arguments list in C, letting programs use the
    // name by which they were invoked in their execution. It’s often convenient to have access to
    // the program name in case you want to print it in messages or change behavior of the program
    // based on what command line alias was used to invoke the program. But for the purposes of
    // this chapter, we’ll ignore it and save only the two arguments we need.

    // if let [_program, _query, _filename] = &args[..] {
    //     println!("Program is {}", _program);
    //     println!("Searching for {}", _query);
    //     println!("In file {}", _filename);
    // } else {
    //     println!("Missing args...");
    // };

    // =======================================
    // Saving the Argument Values in Variables
    // =======================================
    // Printing the value of the vector of arguments illustrated that the program is able to access the
    // values specified as command line arguments. Now we need to save the values of the two arguments
    // in variables so we can use the values throughout the rest of the program. We do that in Listing
    // 12-2.

    // let query = &args[1];
    // let filename = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", filename);

    // As we saw when we printed the vector, the program’s name takes up the first value in the
    // vector at args[0], so we’re starting at index 1. The first argument minigrep takes is the
    // string we’re searching for, so we put a reference to the first argument in the variable
    // query. The second argument will be the filename, so we put a reference to the second
    // argument in the variable filename.

    // We temporarily print the values of these variables to prove that the code is working as we
    // intend. Let’s run this program again with the arguments test and sample.txt:

    // $ cargo run test sample.txt

    // Great, the program is working! The values of the arguments we need are being saved into the
    // right variables. Later we’ll add some error handling to deal with certain potential
    // erroneous situations, such as when the user provides no arguments; for now, we’ll ignore
    // that situation and work on adding file-reading capabilities instead.

    // =======================================
    // Reading a File

    // Now we’ll add functionality to read the file that is specified in the filename command line
    // argument. First, we need a sample file to test it with: the best kind of file to use to make
    // sure minigrep is working is one with a small amount of text over multiple lines with some
    // repeated words. Listing 12-3 has an Emily Dickinson poem that will work well! Create a file
    // called poem.txt at the root level of your project, and enter the poem “I’m Nobody! Who are you?”

    // With the text in place, edit src/main.rs and add code to read the file, as shown in Listing
    // 12-4.

    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);

    // Great! The code read and then printed the contents of the file. But the code has a few
    // flaws. The main function has multiple responsibilities: generally, functions are clearer and
    // easier to maintain if each function is responsible for only one idea. The other problem is
    // that we’re not handling errors as well as we could. The program is still small, so these
    // flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly.
    // It’s good practice to begin refactoring early on when developing a program, because it’s
    // much easier to refactor smaller amounts of code. We’ll do that next.

    // =======================================
    // Refactoring to Improve Modularity and Error Handling
    // https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
    // =======================================

    // To improve our program, we’ll fix four problems that have to do with the program’s structure and
    // how it’s handling potential errors.

    // First, our main function now performs two tasks: it parses arguments and reads files. For such a
    // small function, this isn’t a major problem. However, if we continue to grow our program inside
    // main, the number of separate tasks the main function handles will increase. As a function gains
    // responsibilities, it becomes more difficult to reason about, harder to test, and harder to
    // change without breaking one of its parts. It’s best to separate functionality so each function
    // is responsible for one task.

    // This issue also ties into the second problem: although query and filename are configuration
    // variables to our program, variables like contents are used to perform the program’s logic. The
    // longer main becomes, the more variables we’ll need to bring into scope; the more variables we
    // have in scope, the harder it will be to keep track of the purpose of each. It’s best to group
    // the configuration variables into one structure to make their purpose clear.

    // The third problem is that we’ve used expect to print an error message when reading the file
    // fails, but the error message just prints Something went wrong reading the file. Reading a file
    // can fail in a number of ways: for example, the file could be missing, or we might not have
    // permission to open it. Right now, regardless of the situation, we’d print the Something went
    // wrong reading the file error message, which wouldn’t give the user any information!

    // Fourth, we use expect repeatedly to handle different errors, and if the user runs our program
    // without specifying enough arguments, they’ll get an index out of bounds error from Rust that
    // doesn’t clearly explain the problem. It would be best if all the error-handling code were in one
    // place so future maintainers had only one place to consult in the code if the error-handling
    // logic needed to change. Having all the error-handling code in one place will also ensure that
    // we’re printing messages that will be meaningful to our end users.

    // Let’s address these four problems by refactoring our project.

    // =======================================
    // Separation of Concerns for Binary Projects
    // =======================================
    // The organizational problem of allocating responsibility for multiple tasks to the main function is
    // common to many binary projects. As a result, the Rust community has developed a process to use as a
    // guideline for splitting the separate concerns of a binary program when main starts getting large.
    // The process has the following steps:

    // Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.  As long as
    // your command line parsing logic is small, it can remain in main.rs.  When the command line parsing
    // logic starts getting complicated, extract it from main.rs and move it to lib.rs.  The
    // responsibilities that remain in the main function after this process should be limited to the
    // following:

    // Calling the command line parsing logic with the argument values Setting up any other configuration
    // Calling a run function in lib.rs Handling the error if run returns an error This pattern is about
    // separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the
    // task at hand. Because you can’t test the main function directly, this structure lets you test all
    // of your program’s logic by moving it into functions in lib.rs. The only code that remains in
    // main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by
    // following this process.

    // =======================================
    // Extracting the Argument Parser
    // =======================================
    // We’ll extract the functionality for parsing arguments into a function that main will call to
    // prepare for moving the command line parsing logic to src/lib.rs. Listing 12-5 shows the new start
    // of main that calls a new function parse_config, which we’ll define in src/main.rs for the moment.

    // let args: Vec<String> = env::args().collect();

    // We’ve updated main so it places the instance of Config returned by parse_config into a variable
    // named config, and we updated the code that previously used the separate query and filename
    // variables so it now uses the fields on the Config struct instead.

    // Now our code more clearly conveys that query and filename are related and that their purpose is
    // to configure how the program will work. Any code that uses these values knows to find them in
    // the config instance in the fields named for their purpose.

    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    // We’re still collecting the command line arguments into a vector, but instead of assigning
    // the argument value at index 1 to the variable query and the argument value at index 2 to the
    // variable filename within the main function, we pass the whole vector to the parse_config
    // function. The parse_config function then holds the logic that determines which argument goes
    // in which variable and passes the values back to main. We still create the query and filename
    // variables in main, but main no longer has the responsibility of determining how the command
    // line arguments and variables correspond.

    // This rework may seem like overkill for our small program, but we’re refactoring in small,
    // incremental steps. After making this change, run the program again to verify that the argument
    // parsing still works. It’s good to check your progress often, to help identify the cause of
    // problems when they occur.

    // =======================================
    // Grouping Configuration Values
    // =======================================
    // We can take another small step to improve the parse_config function further. At the moment,
    // we’re returning a tuple, but then we immediately break that tuple into individual parts again.
    // This is a sign that perhaps we don’t have the right abstraction yet.

    // Another indicator that shows there’s room for improvement is the config part of parse_config,
    // which implies that the two values we return are related and are both part of one configuration
    // value. We’re not currently conveying this meaning in the structure of the data other than by
    // grouping the two values into a tuple; we could put the two values into one struct and give each
    // of the struct fields a meaningful name. Doing so will make it easier for future maintainers of
    // this code to understand how the different values relate to each other and what their purpose is.

    // Note: Using primitive values when a complex type would be more appropriate is an anti-pattern
    // known as primitive obsession.
}

// We’ve added a struct named Config defined to have fields named query and filename. The signature
// of parse_config now indicates that it returns a Config value. In the body of parse_config, where
// we used to return string slices that reference String values in args, we now define Config to
// contain owned String values. The args variable in main is the owner of the argument values and
// is only letting the parse_config function borrow them, which means we’d violate Rust’s borrowing
// rules if Config tried to take ownership of the values in args.

// We could manage the String data in a number of different ways, but the easiest, though somewhat
// inefficient, route is to call the clone method on the values. This will make a full copy of the
// data for the Config instance to own, which takes more time and memory than storing a reference
// to the string data. However, cloning the data also makes our code very straightforward because
// we don’t have to manage the lifetimes of the references; in this circumstance, giving up a
// little performance to gain simplicity is a worthwhile trade-off.

// The Trade-Offs of Using clone
// There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because
// of its runtime cost. In Chapter 13, you’ll learn how to use more efficient methods in this type
// of situation. But for now, it’s okay to copy a few strings to continue making progress because
// you’ll make these copies only once and your filename and query string are very small. It’s
// better to have a working program that’s a bit inefficient than to try to hyperoptimize code on
// your first pass. As you become more experienced with Rust, it’ll be easier to start with the
// most efficient solution, but for now, it’s perfectly acceptable to call clone.

// =======================================
// Creating a Constructor for Config
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#creating-a-constructor-for-config
// =======================================

// struct Config {
//     query: String,
//     filename: String,
// }

// So far, we’ve extracted the logic responsible for parsing the command line arguments from main
// and placed it in the parse_config function. Doing so helped us to see that the query and
// filename values were related and that relationship should be conveyed in our code. We then added
// a Config struct to name the related purpose of query and filename and to be able to return the
// values’ names as struct field names from the parse_config function.

// So now that the purpose of the parse_config function is to create a Config instance, we can
// change parse_config from a plain function to a function named new that is associated with the
// Config struct. Making this change will make the code more idiomatic. We can create instances of
// types in the standard library, such as String, by calling String::new. Similarly, by changing
// parse_config into a new function associated with Config, we’ll be able to create instances of
// Config by calling Config::new. Listing 12-7 shows the changes we need to make.

// impl Config {
//     fn new(args: &[String]) -> Config {
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Config { query, filename }
//     }
// }

// =======================================
// Fixing the Error Handling
// =======================================
// Now we’ll work on fixing our error handling. Recall that attempting to access the values in the
// args vector at index 1 or index 2 will cause the program to panic if the vector contains fewer
// than three items. Try running the program without any arguments; it will look like this:

// $ cargo run
// Compiling minigrep v0.1.0 (file:///projects/minigrep)
//  Finished dev [unoptimized + debuginfo] target(s) in 0.0s
//   Running `target/debug/minigrep`
// thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:27:21
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

// The line index out of bounds: the len is 1 but the index is 1 is an error message intended for
// programmers. It won’t help our end users understand what happened and what they should do instead.
// Let’s fix that now.

// =======================================
// Improving the Error Message
// =======================================

// In Listing 12-8, we add a check in the new function that will verify that the slice is long
// enough before accessing index 1 and 2. If the slice isn’t long enough, the program panics and
// displays a better error message than the index out of bounds message.

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Config { query, filename }
//     }
// }

// This code is similar to the Guess::new function we wrote in Listing 9-10, where we called panic!
// when the value argument was out of the range of valid values. Instead of checking for a range of
// values here, we’re checking that the length of args is at least 3 and the rest of the function
// can operate under the assumption that this condition has been met. If args has fewer than three
// items, this condition will be true, and we call the panic! macro to end the program immediately.

// This output is better: we now have a reasonable error message. However, we also have extraneous
// information we don’t want to give to our users. Perhaps using the technique we used in Listing
// 9-10 isn’t the best to use here: a call to panic! is more appropriate for a programming problem
// than a usage problem, as discussed in Chapter 9. Instead, we can use the other technique you
// learned about in Chapter 9—returning a Result that indicates either success or an error.

// =======================================
// Returning a Result from new Instead of Calling panic!
// =======================================
// We can instead return a Result value that will contain a Config instance in the successful case
// and will describe the problem in the error case. When Config::new is communicating to main, we
// can use the Result type to signal there was a problem. Then we can change main to convert an Err
// variant into a more practical error for our users without the surrounding text about thread
// 'main' and RUST_BACKTRACE that a call to panic! causes.

// Listing 12-9 shows the changes we need to make to the return value of Config::new and the body
// of the function needed to return a Result. Note that this won’t compile until we update main as
// well, which we’ll do in the next listing.

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

// Our new function now returns a Result with a Config instance in the success case and a &'static
// str in the error case. Recall from “The Static Lifetime” section in Chapter 10 that &'static str
// is the type of string literals, which is our error message type for now.

// We’ve made two changes in the body of the new function: instead of calling panic! when the user
// doesn’t pass enough arguments, we now return an Err value, and we’ve wrapped the Config return
// value in an Ok. These changes make the function conform to its new type signature.

// Returning an Err value from Config::new allows the main function to handle the Result value
// returned from the new function and exit the process more cleanly in the error case.

// =======================================
// Calling Config::new and Handling Errors
// =======================================
// To handle the error case and print a user-friendly message, we need to update main to handle the
// Result being returned by Config::new, as shown in Listing 12-10. We’ll also take the
// responsibility of exiting the command line tool with a nonzero error code from panic! and
// implement it by hand. A nonzero exit status is a convention to signal to the process that called
// our program that the program exited with an error state.

// In this listing, we’ve used a method we haven’t covered before: unwrap_or_else, which is defined
// on Result<T, E> by the standard library. Using unwrap_or_else allows us to define some custom,
// non-panic! error handling. If the Result is an Ok value, this method’s behavior is similar to
// unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this
// method calls the code in the closure, which is an anonymous function we define and pass as an
// argument to unwrap_or_else. We’ll cover closures in more detail in Chapter 13. For now, you just
// need to know that unwrap_or_else will pass the inner value of the Err, which in this case is the
// static string not enough arguments that we added in Listing 12-9, to our closure in the argument
// err that appears between the vertical pipes. The code in the closure can then use the err value
// when it runs.

// We’ve added a new use line to bring process from the standard library into scope. The code in
// the closure that will be run in the error case is only two lines: we print the err value and
// then call process::exit. The process::exit function will stop the program immediately and return
// the number that was passed as the exit status code. This is similar to the panic!-based handling
// we used in Listing 12-8, but we no longer get all the extra output.

// =======================================
// Extracting Logic from main
// =======================================
// Now that we’ve finished refactoring the configuration parsing, let’s turn to the program’s
// logic. As we stated in “Separation of Concerns for Binary Projects”, we’ll extract a function
// named run that will hold all the logic currently in the main function that isn’t involved with
// setting up configuration or handling errors. When we’re done, main will be concise and easy to
// verify by inspection, and we’ll be able to write tests for all the other logic.

// Listing 12-11 shows the extracted run function. For now, we’re just making the small,
// incremental improvement of extracting the function. We’re still defining the function in
// src/main.rs.

// fn run(config: Config) {
//     let contents =
//         fs::read_to_string(config.filename).expect("Something went wrong reading the file");

//     println!("Query {}", config.query);
//     println!("With text:\n{}", contents);
// }

// The run function now contains all the remaining logic from main, starting from reading the file.
// The run function takes the Config instance as an argument.

// =======================================
// Returning Errors from the run Function
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#returning-errors-from-the-run-function
// =======================================
// With the remaining program logic separated into the run function, we can improve the error
// handling, as we did with Config::new in Listing 12-9. Instead of allowing the program to panic
// by calling expect, the run function will return a Result<T, E> when something goes wrong. This
// will let us further consolidate into main the logic around handling errors in a user-friendly
// way. Listing 12-12 shows the changes we need to make to the signature and body of run.

// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.filename)?;

//     println!("Query {}", config.query);
//     println!("With text:\n{}", contents);

//     Ok(())
// }

// We’ve made three significant changes here. First, we changed the return type of the run function
// to Result<(), Box<dyn Error>>. This function previously returned the unit type, (), and we keep
// that as the value returned in the Ok case.

// For the error type, we used the trait object Box<dyn Error> (and we’ve brought std::error::Error
// into scope with a use statement at the top). We’ll cover trait objects in Chapter 17. For now,
// just know that Box<dyn Error> means the function will return a type that implements the Error
// trait, but we don’t have to specify what particular type the return value will be. This gives us
// flexibility to return error values that may be of different types in different error cases. The
// dyn keyword is short for “dynamic.”

// Second, we’ve removed the call to expect in favor of the ? operator, as we talked about in
// Chapter 9. Rather than panic! on an error, ? will return the error value from the current
// function for the caller to handle.

// Third, the run function now returns an Ok value in the success case. We’ve declared the run
// function’s success type as () in the signature, which means we need to wrap the unit type value
// in the Ok value. This Ok(()) syntax might look a bit strange at first, but using () like this is
// the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t
// return a value we need.

// Rust tells us that our code ignored the Result value and the Result value might indicate that an
// error occurred. But we’re not checking to see whether or not there was an error, and the
// compiler reminds us that we probably meant to have some error-handling code here! Let’s rectify
// that problem now.

// =======================================
// Handling Errors Returned from run in main
// =======================================
// We’ll check for errors and handle them using a technique similar to one we used with Config::new
// in Listing 12-10, but with a slight difference:

// We use if let rather than unwrap_or_else to check whether run returns an Err value and call
// process::exit(1) if it does. The run function doesn’t return a value that we want to unwrap in
// the same way that Config::new returns the Config instance. Because run returns () in the success
// case, we only care about detecting an error, so we don’t need unwrap_or_else to return the
// unwrapped value because it would only be ().

// The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print
// the error and exit.

// =======================================
// Splitting Code into a Library Crate
// =======================================
// Our minigrep project is looking good so far! Now we’ll split the src/main.rs file and put some
// code into the src/lib.rs file so we can test it and have a src/main.rs file with fewer
// responsibilities.

// Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:

// The run function definition
// The relevant use statements
// The definition of Config
// The Config::new function definition

// The contents of src/lib.rs should have the signatures shown in Listing 12-13 (we’ve omitted the
// bodies of the functions for brevity). Note that this won’t compile until we modify src/main.rs
// in Listing 12-14.

// We add a use minigrep::Config line to bring the Config type from the library crate into the
// binary crate’s scope, and we prefix the run function with our crate name. Now all the
// functionality should be connected and should work. Run the program with cargo run and make sure
// everything works correctly.

// Whew! That was a lot of work, but we’ve set ourselves up for success in the future. Now it’s
// much easier to handle errors, and we’ve made the code more modular. Almost all of our work will
// be done in src/lib.rs from here on out.

// Let’s take advantage of this newfound modularity by doing something that would have been
// difficult with the old code but is easy with the new code: we’ll write some tests!
