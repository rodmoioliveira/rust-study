// =======================================
// The Iterator Trait and the next Method
// =======================================
// All iterators implement a trait named Iterator that is defined in the standard library. The
// definition of the trait looks like this:

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// Notice this definition uses some new syntax: type Item and Self::Item, which are defining an
// associated type with this trait. We’ll talk about associated types in depth in Chapter 19. For
// now, all you need to know is that this code says implementing the Iterator trait requires that
// you also define an Item type, and this Item type is used in the return type of the next method.
// In other words, the Item type will be the type returned from the iterator.

// The Iterator trait only requires implementors to define one method: the next method, which
// returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns
// None.

// We can call the next method on iterators directly; Listing 13-15 demonstrates what values are
// returned from repeated calls to next on the iterator created from the vector.

// Note that we needed to make v1_iter mutable: calling the next method on an iterator changes
// internal state that the iterator uses to keep track of where it is in the sequence. In other
// words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the
// iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took
// ownership of v1_iter and made it mutable behind the scenes.

// Also note that the values we get from the calls to next are immutable references to the values
// in the vector. The iter method produces an iterator over immutable references. If we want to
// create an iterator that takes ownership of v1 and returns owned values, we can call into_iter
// instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut
// instead of iter.

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

// =======================================
// Methods that Consume the Iterator
// =======================================
// The Iterator trait has a number of different methods with default implementations provided by
// the standard library; you can find out about these methods by looking in the standard library
// API documentation for the Iterator trait. Some of these methods call the next method in their
// definition, which is why you’re required to implement the next method when implementing the
// Iterator trait.

// Methods that call next are called consuming adaptors, because calling them uses up the iterator.
// One example is the sum method, which takes ownership of the iterator and iterates through the
// items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds
// each item to a running total and returns the total when iteration is complete. Listing 13-16 has
// a test illustrating a use of the sum method:

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

// We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the
// iterator we call it on.

// =======================================
// Using Closures that Capture Their Environment
// =======================================
// Now that we’ve introduced iterators, we can demonstrate a common use of closures that capture
// their environment by using the filter iterator adaptor. The filter method on an iterator takes a
// closure that takes each item from the iterator and returns a Boolean. If the closure returns
// true, the value will be included in the iterator produced by filter. If the closure returns
// false, the value won’t be included in the resulting iterator.

// In Listing 13-19, we use filter with a closure that captures the shoe_size variable from its
// environment to iterate over a collection of Shoe struct instances. It will return only shoes
// that are the specified size.

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let size: u32 = 10;
        let in_my_size = shoes_in_my_size(shoes, size);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// The shoes_in_my_size function takes ownership of a vector of shoes and a shoe size as
// parameters. It returns a vector containing only shoes of the specified size.

// In the body of shoes_in_my_size, we call into_iter to create an iterator that takes ownership of
// the vector. Then we call filter to adapt that iterator into a new iterator that only contains
// elements for which the closure returns true.

// The closure captures the shoe_size parameter from the environment and compares the value with
// each shoe’s size, keeping only shoes of the size specified. Finally, calling collect gathers the
// values returned by the adapted iterator into a vector that’s returned by the function.

// The test shows that when we call shoes_in_my_size, we get back only shoes that have the same
// size as the value we specified.

// =======================================
// Creating Our Own Iterators with the Iterator Trait
// =======================================
// We’ve shown that you can create an iterator by calling iter, into_iter, or iter_mut on a vector.
// You can create iterators from the other collection types in the standard library, such as hash
// map. You can also create iterators that do anything you want by implementing the Iterator trait
// on your own types. As previously mentioned, the only method you’re required to provide a
// definition for is the next method. Once you’ve done that, you can use all other methods that
// have default implementations provided by the Iterator trait!

// To demonstrate, let’s create an iterator that will only ever count from 1 to 5. First, we’ll
// create a struct to hold some values. Then we’ll make this struct into an iterator by
// implementing the Iterator trait and using the values in that implementation.

// Listing 13-20 has the definition of the Counter struct and an associated new function to create
// instances of Counter:

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn value(&self) -> u32 {
        self.count
    }
}

// The Counter struct has one field named count. This field holds a u32 value that will keep track
// of where we are in the process of iterating from 1 to 5. The count field is private because we
// want the implementation of Counter to manage its value. The new function enforces the behavior
// of always starting new instances with a value of 0 in the count field.

// Next, we’ll implement the Iterator trait for our Counter type by defining the body of the next
// method to specify what we want to happen when this iterator is used, as shown in Listing 13-21:

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// We set the associated Item type for our iterator to u32, meaning the iterator will return u32
// values. Again, don’t worry about associated types yet, we’ll cover them in Chapter 19.

// We want our iterator to add 1 to the current state, so we initialized count to 0 so it would
// return 1 first. If the value of count is less than 5, next will increment count and return the
// current value wrapped in Some. Once count is 5, our iterator will stop incrementing count and
// always return None.

// =======================================
// Using Our Counter Iterator’s next Method
// =======================================
// Once we’ve implemented the Iterator trait, we have an iterator! Listing 13-22 shows a test
// demonstrating that we can use the iterator functionality of our Counter struct by calling the
// next method on it directly, just as we did with the iterator created from a vector in Listing
// 13-15.

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.value(), 0);
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.value(), 1);
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.value(), 2);
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.value(), 3);
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.value(), 4);
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.value(), 5);
        assert_eq!(counter.next(), None);
    }
}

// This test creates a new Counter instance in the counter variable and then calls next repeatedly,
// verifying that we have implemented the behavior we want this iterator to have: returning the
// values from 1 to 5.

// =======================================
// Using Other Iterator Trait Methods
// =======================================
// We implemented the Iterator trait by defining the next method, so we can now use any Iterator
// trait method’s default implementations as defined in the standard library, because they all use
// the next method’s functionality.

// For example, if for some reason we wanted to take the values produced by an instance of Counter,
// pair them with values produced by another Counter instance after skipping the first value,
// multiply each pair together, keep only those results that are divisible by 3, and add all the
// resulting values together, we could do so, as shown in the test in Listing 13-23:

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// Note that zip produces only four pairs; the theoretical fifth pair (5, None) is never produced
// because zip returns None when either of its input iterators return None.

// All of these method calls are possible because we specified how the next method works, and the
// standard library provides default implementations for other methods that call next.
