pub mod aggregator {
    // Here, we declare a trait using the trait keyword and then the trait’s name, which is Summary in
    // this case. Inside the curly brackets, we declare the method signatures that describe the
    // behaviors of the types that implement this trait, which in this case is fn summarize(&self) ->
    // String.

    // After the method signature, instead of providing an implementation within curly brackets, we use
    // a semicolon. Each type implementing this trait must provide its own custom behavior for the body
    // of the method. The compiler will enforce that any type that has the Summary trait will have the
    // method summarize defined with this signature exactly.

    // A trait can have multiple methods in its body: the method signatures are listed one per line and
    // each line ends in a semicolon.

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String;
    }

    pub trait DefaultSummary {
        fn def_summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // To use a default implementation to summarize instances of NewsArticle instead of defining a
    // custom implementation, we specify an empty impl block with impl Summary for NewsArticle {}.

    // Even though we’re no longer defining the summarize method on NewsArticle directly, we’ve
    // provided a default implementation and specified that NewsArticle implements the Summary trait.
    // As a result, we can still call the summarize method on an instance of NewsArticle.

    // =======================================
    // Implementing a Trait on a Type
    // =======================================
    // Now that we’ve defined the desired behavior using the Summary trait, we can implement it on the
    // types in our media aggregator. Listing 10-13 shows an implementation of the Summary trait on the
    // NewsArticle struct that uses the headline, the author, and the location to create the return
    // value of summarize. For the Tweet struct, we define summarize as the username followed by the
    // entire text of the tweet, assuming that tweet content is already limited to 280 characters.

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl DefaultSummary for NewsArticle {}
    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("author {}", self.author)
        }
        fn summarize(&self) -> String {
            format!(
                "{}, {} ({})",
                self.headline,
                self.summarize_author(),
                self.location
            )
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl DefaultSummary for Tweet {}
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("username {}", self.username)
        }
        fn summarize(&self) -> String {
            format!("{}: {}", self.summarize_author(), self.content)
        }
    }

    // Implementing a trait on a type is similar to implementing regular methods. The difference is
    // that after impl, we put the trait name that we want to implement, then use the for keyword, and
    // then specify the name of the type we want to implement the trait for. Within the impl block, we
    // put the method signatures that the trait definition has defined. Instead of adding a semicolon
    // after each signature, we use curly brackets and fill in the method body with the specific
    // behavior that we want the methods of the trait to have for the particular type.

    // Note that because we defined the Summary trait and the NewsArticle and Tweet types in the
    // same lib.rs in Listing 10-13, they’re all in the same scope. Let’s say this lib.rs is for a
    // crate we’ve called aggregator and someone else wants to use our crate’s functionality to
    // implement the Summary trait on a struct defined within their library’s scope. They would
    // need to bring the trait into their scope first. They would do so by specifying use
    // aggregator::Summary;, which then would enable them to implement Summary for their type. The
    // Summary trait would also need to be a public trait for another crate to implement it, which
    // it is because we put the pub keyword before trait in Listing 10-12.

    // One restriction to note with trait implementations is that we can implement a trait on a type
    // only if either the trait or the type is local to our crate. For example, we can implement
    // standard library traits like Display on a custom type like Tweet as part of our aggregator crate
    // functionality, because the type Tweet is local to our aggregator crate. We can also implement
    // Summary on Vec<T> in our aggregator crate, because the trait Summary is local to our aggregator
    // crate.

    // But we can’t implement external traits on external types. For example, we can’t implement the
    // Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are defined in
    // the standard library and aren’t local to our aggregator crate. This restriction is part of a
    // property of programs called coherence, and more specifically the orphan rule, so named because
    // the parent type is not present. This rule ensures that other people’s code can’t break your code
    // and vice versa. Without the rule, two crates could implement the same trait for the same type,
    // and Rust wouldn’t know which implementation to use.

    // =======================================
    // Default Implementations
    // https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations
    // =======================================
    // Sometimes it’s useful to have default behavior for some or all of the methods in a trait
    // instead of requiring implementations for all methods on every type. Then, as we implement
    // the trait on a particular type, we can keep or override each method’s default behavior.

    // Listing 10-14 shows how to specify a default string for the summarize method of the Summary
    // trait instead of only defining the method signature, as we did in Listing 10-12.

    // Creating a default implementation for summarize doesn’t require us to change anything about
    // the implementation of Summary on Tweet in Listing 10-13. The reason is that the syntax for
    // overriding a default implementation is the same as the syntax for implementing a trait
    // method that doesn’t have a default implementation.

    // Default implementations can call other methods in the same trait, even if those other
    // methods don’t have a default implementation. In this way, a trait can provide a lot of
    // useful functionality and only require implementors to specify a small part of it. For
    // example, we could define the Summary trait to have a summarize_author method whose
    // implementation is required, and then define a summarize method that has a default
    // implementation that calls the summarize_author method:

    // Note that it isn’t possible to call the default implementation from an overriding
    // implementation of that same method.

    // =======================================
    // Traits as Parameters
    // =======================================
    // Now that you know how to define and implement traits, we can explore how to use traits to define
    // functions that accept many different types.

    // For example, in Listing 10-13, we implemented the Summary trait on the NewsArticle and Tweet
    // types. We can define a notify function that calls the summarize method on its item parameter,
    // which is of some type that implements the Summary trait. To do this, we can use the impl Trait
    // syntax, like this:

    pub fn notify(item: &impl Summary) {
        println!("{}", item.summarize());
    }

    // Instead of a concrete type for the item parameter, we specify the impl keyword and the trait
    // name. This parameter accepts any type that implements the specified trait. In the body of
    // notify, we can call any methods on item that come from the Summary trait, such as summarize.
    // We can call notify and pass in any instance of NewsArticle or Tweet. Code that calls the
    // function with any other type, such as a String or an i32, won’t compile because those types
    // don’t implement Summary.

    // =======================================
    // Trait Bound Syntax
    // =======================================

    // The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer
    // form, which is called a trait bound; it looks like this:

    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // This longer form is equivalent to the example in the previous section but is more verbose. We place trait bounds with the declaration of the generic type parameter after a colon and inside angle brackets.

    // The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait bound syntax can express more complexity in other cases. For example, we can have two parameters that implement Summary. Using the impl Trait syntax looks like this:

    pub fn notify_differ_types(item1: &impl Summary, item2: &impl Summary) {
        println!(
            "Breaking news! {}, {}",
            item1.summarize(),
            item2.summarize()
        );
    }

    // If we wanted this function to allow item1 and item2 to have different types, using impl Trait would be appropriate (as long as both types implement Summary). If we wanted to force both parameters to have the same type, that’s only possible to express using a trait bound, like this:

    pub fn notify_same_type<T: Summary>(item1: &T, item2: &T) {
        println!(
            "Breaking news! {}, {}",
            item1.summarize(),
            item2.summarize()
        );
    }

    // The generic type T specified as the type of the item1 and item2 parameters constrains the function such that the concrete type of the value passed as an argument for item1 and item2 must be the same.

    // =======================================
    // Specifying Multiple Trait Bounds with the + Syntax
    // =======================================

    // We can also specify more than one trait bound. Say we wanted notify to use display formatting on
    // item as well as the summarize method: we specify in the notify definition that item must
    // implement both Display and Summary. We can do so using the + syntax:

    // pub fn notify(item: &(impl Summary + Display)) {

    // The + syntax is also valid with trait bounds on generic types:

    // pub fn notify<T: Summary + Display>(item: &T) {

    // With the two trait bounds specified, the body of notify can call summarize and use {} to format
    // item.

    // =======================================
    // Clearer Trait Bounds with where Clauses
    // =======================================
    // Using too many trait bounds has its downsides. Each generic has its own trait bounds, so
    // functions with multiple generic type parameters can contain lots of trait bound information
    // between the function’s name and its parameter list, making the function signature hard to
    // read. For this reason, Rust has alternate syntax for specifying trait bounds inside a where
    // clause after the function signature. So instead of writing this:

    use core::fmt::{Debug, Display};

    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        8
    }

    // we can use a where clause, like this:

    fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        8
    }
    // This function’s signature is less cluttered: the function name, parameter list, and return
    // type are close together, similar to a function without lots of trait bounds.

    // =======================================
    // Returning Types that Implement Traits
    // =======================================
    // We can also use the impl Trait syntax in the return position to return a value of some type
    // that implements a trait, as shown here:

    pub fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }

    // By using impl Summary for the return type, we specify that the returns_summarizable function
    // returns some type that implements the Summary trait without naming the concrete type. In this
    // case, returns_summarizable returns a Tweet, but the code calling this function doesn’t know
    // that.

    // The ability to return a type that is only specified by the trait it implements is especially
    // useful in the context of closures and iterators, which we cover in Chapter 13. Closures and
    // iterators create types that only the compiler knows or types that are very long to specify. The
    // impl Trait syntax lets you concisely specify that a function returns some type that implements
    // the Iterator trait without needing to write out a very long type.

    // However, you can only use impl Trait if you’re returning a single type. For example, this code
    // that returns either a NewsArticle or a Tweet with the return type specified as impl Summary
    // wouldn’t work

    // Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the
    // impl Trait syntax is implemented in the compiler. We’ll cover how to write a function with
    // this behavior in the “Using Trait Objects That Allow for Values of Different Types” section
    // of Chapter 17.

    // =======================================
    // Using Trait Bounds to Conditionally Implement Methods
    // =======================================

    // By using a trait bound with an impl block that uses generic type parameters, we can implement
    // methods conditionally for types that implement the specified traits. For example, the type
    // Pair<T> in Listing 10-16 always implements the new function. But Pair<T> only implements the
    // cmp_display method if its inner type T implements the PartialOrd trait that enables comparison
    // and the Display trait that enables printing.

    pub struct Pair<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Pair<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        pub fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // We can also conditionally implement a trait for any type that implements another trait.
    // Implementations of a trait on any type that satisfies the trait bounds are called blanket
    // implementations and are extensively used in the Rust standard library. For example, the
    // standard library implements the ToString trait on any type that implements the Display
    // trait. The impl block in the standard library looks similar to this code:

    // impl<T: Display> ToString for T {
    //     // --snip--
    // }

    // Because the standard library has this blanket implementation, we can call the to_string method
    // defined by the ToString trait on any type that implements the Display trait. For example, we can
    // turn integers into their corresponding String values like this because integers implement
    // Display:

    // let s = 3.to_string();

    // Blanket implementations appear in the documentation for the trait in the “Implementors” section.

    // Traits and trait bounds let us write code that uses generic type parameters to reduce
    // duplication but also specify to the compiler that we want the generic type to have particular
    // behavior. The compiler can then use the trait bound information to check that all the concrete
    // types used with our code provide the correct behavior. In dynamically typed languages, we would
    // get an error at runtime if we called a method on a type which didn’t define the method. But Rust
    // moves these errors to compile time so we’re forced to fix the problems before our code is even
    // able to run. Additionally, we don’t have to write code that checks for behavior at runtime
    // because we’ve already checked at compile time. Doing so improves performance without having to
    // give up the flexibility of generics.

    // Another kind of generic that we’ve already been using is called lifetimes. Rather than ensuring
    // that a type has the behavior we want, lifetimes ensure that references are valid as long as we
    // need them to be. Let’s look at how lifetimes do that.
}
