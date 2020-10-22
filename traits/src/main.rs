// =======================================
// Traits: Defining Shared Behavior
// https://doc.rust-lang.org/book/ch10-02-traits.html#traits-defining-shared-behavior
// =======================================
// A trait tells the Rust compiler about functionality a particular type has and can share with
// other types. We can use traits to define shared behavior in an abstract way. We can use trait
// bounds to specify that a generic can be any type that has certain behavior.

// Note: Traits are similar to a feature often called interfaces in other languages, although with
// some differences.

// =======================================
// Defining a Trait
// =======================================
// A type’s behavior consists of the methods we can call on that type. Different types share
// the same behavior if we can call the same methods on all of those types. Trait definitions
// are a way to group method signatures together to define a set of behaviors necessary to
// accomplish some purpose.

// For example, let’s say we have multiple structs that hold various kinds and amounts of text: a
// NewsArticle struct that holds a news story filed in a particular location and a Tweet that can
// have at most 280 characters along with metadata that indicates whether it was a new tweet, a
// retweet, or a reply to another tweet.

// We want to make a media aggregator library that can display summaries of data that might be
// stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we
// need to request that summary by calling a summarize method on an instance. Listing 10-12 shows
// the definition of a Summary trait that expresses this behavior.

mod lib;
use lib::{Summary, Tweet};

fn main() {
    let my_tweet = Tweet {
        username: String::from("rodolfo"),
        content: String::from("um tweet qualquer..."),
        reply: false,
        retweet: false,
    };
    println!("{}", my_tweet.summarize());
}
