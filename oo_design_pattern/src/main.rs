fn main() {
    // =======================================
    // Implementing an Object-Oriented Design Pattern
    // https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
    // =======================================
    // The state pattern is an object-oriented design pattern. The crux of the pattern is that a value
    // has some internal state, which is represented by a set of state objects, and the value’s
    // behavior changes based on the internal state. The state objects share functionality: in Rust, of
    // course, we use structs and traits rather than objects and inheritance. Each state object is
    // responsible for its own behavior and for governing when it should change into another state. The
    // value that holds a state object knows nothing about the different behavior of the states or when
    // to transition between states.

    // Using the state pattern means when the business requirements of the program change, we won’t
    // need to change the code of the value holding the state or the code that uses the value. We’ll
    // only need to update the code inside one of the state objects to change its rules or perhaps add
    // more state objects. Let’s look at an example of the state design pattern and how to use it in
    // Rust.

    // We’ll implement a blog post workflow in an incremental way. The blog’s final functionality will
    // look like this:

    // A blog post starts as an empty draft.
    // When the draft is done, a review of the post is requested.
    // When the post is approved, it gets published.
    // Only published blog posts return content to print, so unapproved posts can’t accidentally be published.
    // Any other changes attempted on a post should have no effect. For example, if we try to approve a draft blog post before we’ve requested a review, the post should remain an unpublished draft.

    // Listing 17-11 shows this workflow in code form: this is an example usage of the API we’ll
    // implement in a library crate named blog. This won’t compile yet because we haven’t implemented
    // the blog crate yet.

    println!("Hello, world!");
}
