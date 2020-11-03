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

// =======================================
// Comparing Performance: Loops vs. Iterators
// https://doc.rust-lang.org/book/ch13-04-performance.html
// =======================================
// To determine whether to use loops or iterators, you need to know which version of our search
// functions is faster: the version with an explicit for loop or the version with iterators.

// We ran a benchmark by loading the entire contents of The Adventures of Sherlock Holmes by Sir
// Arthur Conan Doyle into a String and looking for the word the in the contents. Here are the
// results of the benchmark on the version of search using the for loop and the version using
// iterators:

// test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
// test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)

// The iterator version was slightly faster! We won’t explain the benchmark code here, because the
// point is not to prove that the two versions are equivalent but to get a general sense of how
// these two implementations compare performance-wise.

// For a more comprehensive benchmark, you should check using various texts of various sizes as the
// contents, different words and words of different lengths as the query, and all kinds of other
// variations. The point is this: iterators, although a high-level abstraction, get compiled down
// to roughly the same code as if you’d written the lower-level code yourself. Iterators are one of
// Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional
// runtime overhead. This is analogous to how Bjarne Stroustrup, the original designer and
// implementor of C++, defines zero-overhead in “Foundations of C++” (2012):

// In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t
// pay for. And further: What you do use, you couldn’t hand code any better.

// As another example, the following code is taken from an audio decoder. The decoding algorithm
// uses the linear prediction mathematical operation to estimate future values based on a linear
// function of the previous samples. This code uses an iterator chain to do some math on three
// variables in scope: a buffer slice of data, an array of 12 coefficients, and an amount by which
// to shift data in qlp_shift. We’ve declared the variables within this example but not given them
// any values; although this code doesn’t have much meaning outside of its context, it’s still a
// concise, real-world example of how Rust translates high-level ideas to low-level code.

// let buffer: &mut [i32];
// let coefficients: [i64; 12];
// let qlp_shift: i16;

// for i in 12..buffer.len() {
//     let prediction = coefficients.iter()
//                                  .zip(&buffer[i - 12..i])
//                                  .map(|(&c, &s)| c * s as i64)
//                                  .sum::<i64>() >> qlp_shift;
//     let delta = buffer[i];
//     buffer[i] = prediction as i32 + delta;
// }

// To calculate the value of prediction, this code iterates through each of the 12 values in
// coefficients and uses the zip method to pair the coefficient values with the previous 12 values
// in buffer. Then, for each pair, we multiply the values together, sum all the results, and shift
// the bits in the sum qlp_shift bits to the right.

// Calculations in applications like audio decoders often prioritize performance most highly. Here,
// we’re creating an iterator, using two adaptors, and then consuming the value. What assembly code
// would this Rust code compile to? Well, as of this writing, it compiles down to the same assembly
// you’d write by hand. There’s no loop at all corresponding to the iteration over the values in
// coefficients: Rust knows that there are 12 iterations, so it “unrolls” the loop. Unrolling is an
// optimization that removes the overhead of the loop controlling code and instead generates
// repetitive code for each iteration of the loop.

// All of the coefficients get stored in registers, which means accessing the values is very fast.
// There are no bounds checks on the array access at runtime. All these optimizations that Rust is
// able to apply make the resulting code extremely efficient. Now that you know this, you can use
// iterators and closures without fear! They make code seem like it’s higher level but don’t impose
// a runtime performance penalty for doing so.

// =======================================
// Summary
// =======================================
// Closures and iterators are Rust features inspired by functional programming language ideas. They
// contribute to Rust’s capability to clearly express high-level ideas at low-level performance.
// The implementations of closures and iterators are such that runtime performance is not affected.
// This is part of Rust’s goal to strive to provide zero-cost abstractions.

// Now that we’ve improved the expressiveness of our I/O project, let’s look at some more features
// of cargo that will help us share the project with the world.
