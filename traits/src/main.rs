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

use traits::aggregator::{self, DefaultSummary, Pair, Summary};

fn main() {
    let my_tweet = aggregator::Tweet {
        username: String::from("rodolfo"),
        content: String::from("um tweet qualquer..."),
        reply: false,
        retweet: false,
    };
    let my_article = aggregator::NewsArticle {
        headline: String::from("Learning Rust!"),
        location: String::from("Rio de Janeiro"),
        author: String::from("Rodolfo Oliveira"),
        content: String::from("is a nice language..."),
    };
    let test = 1;

    // After implementing the trait, we can call the methods on instances of NewsArticle and Tweet in
    // the same way we call regular methods, like this:

    // println!("{}", my_tweet.summarize());
    // println!("{}", my_article.summarize());
    // println!("{}", my_tweet.def_summarize());
    // println!("{}", my_article.def_summarize());

    aggregator::notify(&my_tweet);
    aggregator::notify(&my_article);
    aggregator::notify2(&my_tweet);
    aggregator::notify2(&my_article);
    // aggregator::notify(&test);
    aggregator::notify_same_type(&my_tweet, &my_tweet);
    aggregator::notify_differ_types(&my_article, &my_tweet);

    let n_pair = Pair::new(3, 4);
    let c_pair = Pair::new('v', 'A');
    n_pair.cmp_display();
    c_pair.cmp_display();

    // let x_pair = Pair::new(&my_tweet, &my_tweet);
    // x_pair.cmp_display();
}
