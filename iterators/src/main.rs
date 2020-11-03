// =======================================
// Processing a Series of Items with Iterators
// https://doc.rust-lang.org/book/ch13-02-iterators.html
// =======================================

fn main() {
    // The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator
    // is responsible for the logic of iterating over each item and determining when the sequence has
    // finished. When you use iterators, you don’t have to reimplement that logic yourself.

    // In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the
    // iterator to use it up. For example, the code in Listing 13-13 creates an iterator over the items
    // in the vector v1 by calling the iter method defined on Vec<T>. This code by itself doesn’t do
    // anything useful.

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // Once we’ve created an iterator, we can use it in a variety of ways. In Listing 3-5 in
    // Chapter 3, we used iterators with for loops to execute some code on each item, although we
    // glossed over what the call to iter did until now.

    // The example in Listing 13-14 separates the creation of the iterator from the use of the iterator
    // in the for loop. The iterator is stored in the v1_iter variable, and no iteration takes place at
    // that time. When the for loop is called using the iterator in v1_iter, each element in the
    // iterator is used in one iteration of the loop, which prints out each value.

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // In languages that don’t have iterators provided by their standard libraries, you would
    // likely write this same functionality by starting a variable at index 0, using that variable
    // to index into the vector to get a value, and incrementing the variable value in a loop until
    // it reached the total number of items in the vector.

    // Iterators handle all that logic for you, cutting down on repetitive code you could potentially
    // mess up. Iterators give you more flexibility to use the same logic with many different kinds of
    // sequences, not just data structures you can index into, like vectors. Let’s examine how
    // iterators do that.

    // =======================================
    // Methods that Produce Other Iterators
    // =======================================
    // Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change
    // iterators into different kinds of iterators. You can chain multiple calls to iterator adaptors
    // to perform complex actions in a readable way. But because all iterators are lazy, you have to
    // call one of the consuming adaptor methods to get results from calls to iterator adaptors.

    // Listing 13-17 shows an example of calling the iterator adaptor method map, which takes a closure
    // to call on each item to produce a new iterator. The closure here creates a new iterator in which
    // each item from the vector has been incremented by 1. However, this code produces a warning:

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_mapped = v1.iter().map(|x| x + 1);

    // The code in Listing 13-17 doesn’t do anything; the closure we’ve specified never gets
    // called. The warning reminds us why: iterator adaptors are lazy, and we need to consume the
    // iterator here.

    // To fix this and consume the iterator, we’ll use the collect method, which we used in Chapter 12
    // with env::args in Listing 12-1. This method consumes the iterator and collects the resulting
    // values into a collection data type.

    // In Listing 13-18, we collect the results of iterating over the iterator that’s returned from the
    // call to map into a vector. This vector will end up containing each item from the original vector
    // incremented by 1.

    let v2: Vec<_> = v1_mapped.collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // Because map takes a closure, we can specify any operation we want to perform on each item.
    // This is a great example of how closures let you customize some behavior while reusing the
    // iteration behavior that the Iterator trait provides.
}

// =======================================
// Improving Our I/O Project
// https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
// =======================================
