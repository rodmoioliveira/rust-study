#![allow(unused_variables)]
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

// =======================================
// What Is Ownership?
// =======================================
// Rust’s central feature is ownership. Although the feature is straightforward to explain, it has
// deep implications for the rest of the language.

// All programs have to manage the way they use a computer’s memory while running. Some languages
// have garbage collection that constantly looks for no longer used memory as the program runs; in
// other languages, the programmer must explicitly allocate and free the memory. Rust uses a third
// approach: memory is managed through a system of ownership with a set of rules that the compiler
// checks at compile time. None of the ownership features slow down your program while it’s
// running.

// =======================================
// The Stack and the Heap
// =======================================
// In many programming languages, you don’t have to think about the stack and the heap very often.
// But in a systems programming language like Rust, whether a value is on the stack or the heap has
// more of an effect on how the language behaves and why you have to make certain decisions. Parts
// of ownership will be described in relation to the stack and the heap later in this chapter, so
// here is a brief explanation in preparation.

// Both the stack and the heap are parts of memory that are available to your code to use at
// runtime, but they are structured in different ways. The stack stores values in the order it gets
// them and removes the values in the opposite order. This is referred to as last in, first out.
// Think of a stack of plates: when you add more plates, you put them on top of the pile, and when
// you need a plate, you take one off the top. Adding or removing plates from the middle or bottom
// wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called
// popping off the stack.

// All data stored on the stack must have a known, fixed size. Data with an unknown size at compile
// time or a size that might change must be stored on the heap instead. The heap is less organized:
// when you put data on the heap, you request a certain amount of space. The memory allocator finds
// an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer,
// which is the address of that location. This process is called allocating on the heap and is
// sometimes abbreviated as just allocating. Pushing values onto the stack is not considered
// allocating. Because the pointer is a known, fixed size, you can store the pointer on the stack,
// but when you want the actual data, you must follow the pointer.

// Think of being seated at a restaurant. When you enter, you state the number of people in your
// group, and the staff finds an empty table that fits everyone and leads you there. If someone in
// your group comes late, they can ask where you’ve been seated to find you.

// Pushing to the stack is faster than allocating on the heap because the allocator never has to
// search for a place to store new data; that location is always at the top of the stack.
// Comparatively, allocating space on the heap requires more work, because the allocator must first
// find a big enough space to hold the data and then perform bookkeeping to prepare for the next
// allocation.

// Accessing data in the heap is slower than accessing data on the stack because you have to follow
// a pointer to get there. Contemporary processors are faster if they jump around less in memory.
// Continuing the analogy, consider a server at a restaurant taking orders from many tables. It’s
// most efficient to get all the orders at one table before moving on to the next table. Taking an
// order from table A, then an order from table B, then one from A again, and then one from B again
// would be a much slower process. By the same token, a processor can do its job better if it works
// on data that’s close to other data (as it is on the stack) rather than farther away (as it can
// be on the heap). Allocating a large amount of space on the heap can also take time.

// When your code calls a function, the values passed into the function (including, potentially,
// pointers to data on the heap) and the function’s local variables get pushed onto the stack. When
// the function is over, those values get popped off the stack.

// Keeping track of what parts of code are using what data on the heap, minimizing the amount of
// duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of
// space are all problems that ownership addresses. Once you understand ownership, you won’t need
// to think about the stack and the heap very often, but knowing that managing heap data is why
// ownership exists can help explain why it works the way it does.

fn main() {
    // =======================================
    // Ownership Rules
    // =======================================
    // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through
    // the examples that illustrate them:

    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // =======================================
    // Variable Scope
    // =======================================

    let _immutable_string = "hello";
    // We’ve already seen string literals, where a string value is hardcoded into our program.
    // String literals are convenient, but they aren’t suitable for every situation in which we may
    // want to use text. One reason is that they’re immutable. Another is that not every string
    // value can be known when we write our code: for example, what if we want to take user input
    // and store it? For these situations, Rust has a second string type, String. This type is
    // allocated on the heap and as such is able to store an amount of text that is unknown to us
    // at compile time. You can create a String from a string literal using the from function, like
    // so:
    let mut s = String::from("hello");
    // This kind of string can be mutated:
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    // =======================================
    // Memory and Allocation
    // =======================================
    // In the case of a string literal, we know the contents at compile time, so the text is
    // hardcoded directly into the final executable. This is why string literals are fast and
    // efficient. But these properties only come from the string literal’s immutability.
    // Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose
    // size is unknown at compile time and whose size might change while running the program.

    // With the String type, in order to support a mutable, growable piece of text, we need to
    // allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
    // This means:

    // The memory must be requested from the memory allocator at runtime.  We need a way of
    // returning this memory to the allocator when we’re done with our String.

    // That first part is done by us: when we call String::from, its implementation requests the
    // memory it needs. This is pretty much universal in programming languages.

    // However, the second part is different. In languages with a garbage collector (GC), the GC
    // keeps track and cleans up memory that isn’t being used anymore, and we don’t need to think
    // about it. Without a GC, it’s our responsibility to identify when memory is no longer being
    // used and call code to explicitly return it, just as we did to request it. Doing this
    // correctly has historically been a difficult programming problem. If we forget, we’ll waste
    // memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a
    // bug too. We need to pair exactly one allocate with exactly one free.

    // Rust takes a different path: the memory is automatically returned once the variable that
    // owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a
    // String instead of a string literal:'

    {
        let s = String::from("hello"); // s is valid from this point forward
                                       // do stuff with s
    } // this scope is now over, and s is no longer valid

    // There is a natural point at which we can return the memory our String needs to the
    // allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special
    // function for us. This function is called drop, and it’s where the author of String can put
    // the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    // Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is
    // sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust
    // will be familiar to you if you’ve used RAII patterns.

    // This pattern has a profound impact on the way Rust code is written. It may seem simple right
    // now, but the behavior of code can be unexpected in more complicated situations when we want
    // to have multiple variables use the data we’ve allocated on the heap. Let’s explore some of
    // those situations now.

    // If you’ve heard the terms shallow copy and deep copy while working with other languages, the
    // concept of copying the pointer, length, and capacity without copying the data probably
    // sounds like making a shallow copy. But because Rust also invalidates the first variable,
    // instead of being called a shallow copy, it’s known as a move. In this example, we would say
    // that s1 was moved into s2:

    // let s1 = String::from("hello");
    // let s2 = s1; // move!!! s1 is no loger valid!

    // println!("{}, world!", s1);

    // =======================================
    // Ways Variables and Data Interact: Clone
    // =======================================

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can
    // use a common method called clone. We’ll discuss method syntax in Chapter 5, but because
    // methods are a common feature in many programming languages, you’ve probably seen them
    // before.

    // Here’s an example of the clone method in action:

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // This works just fine and explicitly produces the behavior shown in Figure 4-3, where the
    // heap data does get copied.

    // =======================================
    // Stack-Only Data: Copy
    // =======================================
    // There’s another wrinkle we haven’t talked about yet. This code using integers, part of which
    // was shown in Listing 4-2, works and is valid:

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // But this code seems to contradict what we just learned: we don’t have a call to clone, but x
    // is still valid and wasn’t moved into y.

    // The reason is that types such as integers that have a known size at compile time are stored
    // entirely on the stack, so copies of the actual values are quick to make. That means there’s
    // no reason we would want to prevent x from being valid after we create the variable y. In
    // other words, there’s no difference between deep and shallow copying here, so calling clone
    // wouldn’t do anything different from the usual shallow copying and we can leave it out.

    // Rust has a special annotation called the Copy trait that we can place on types like integers
    // that are stored on the stack (we’ll talk more about traits in Chapter 10). If a type has the
    // Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a
    // type with the Copy trait if the type, or any of its parts, has implemented the Drop trait.
    // If the type needs something special to happen when the value goes out of scope and we add
    // the Copy annotation to that type, we’ll get a compile-time error.

    // So what types are Copy? You can check the documentation for the given type to be sure, but
    // as a general rule, any group of simple scalar values can be Copy, and nothing that requires
    // allocation or is some form of resource is Copy. Here are some of the types that are Copy:

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

    // =======================================
    // Ownership and Functions
    // =======================================
    // The semantics for passing a value to a function are similar to those for assigning a value
    // to a variable. Passing a variable to a function will move or copy, just as assignment does.
    // Listing 4-3 has an example with some annotations showing where variables go into and out of
    // scope.

    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function... and so is no longer valid here
        let x = 5; // x comes into scope
                   // println!("{}", s); don't work!!!!!!!!!!!!!!!!!!
        makes_copy(x); // x would move into the function, but i32 is Copy, so it’s okay to still use x afterward
        println!("{}", x);
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.

    // =======================================
    // References and Borrowing
    // https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    // =======================================
    let s3 = String::from("hello");

    // Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference.
    fn calculate_length(some_string: &String) -> usize {
        some_string.len()
    }

    // These ampersands are references, and they allow you to refer to some value without taking
    // ownership of it.s The &s3 syntax lets us create a reference that refers to the value of s1
    // but does not own it. Because it does not own it, the value it points to will not be dropped
    // when the reference goes out of scope.
    let len = calculate_length(&s3);
    println!("len is {}", len);

    // The scope in which the variable s is valid is the same as any function parameter’s scope,
    // but we don’t drop what the reference points to when it goes out of scope because we don’t
    // have ownership. When functions have references as parameters instead of the actual values,
    // we won’t need to return the values in order to give back ownership, because we never had
    // ownership.

    // We call having references as function parameters borrowing. As in real life, if a person
    // owns something, you can borrow it from them. When you’re done, you have to give it back.

    // =======================================
    // Mutable References
    // =======================================
    let mut s4 = String::from("hello");
    change(&mut s4);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    println!("{}", s4);

    // But mutable references have one big restriction: you can have only one mutable reference to
    // a particular piece of data in a particular scope. This code will fail:

    // let mut s6 = String::from("hello");
    // let r1 = &mut s6;
    // let r2 = &mut s6;
    // println!("{}, {}", r1, r2);

    // This restriction allows for mutation but in a very controlled fashion. It’s something that
    // new Rustaceans struggle with, because most languages let you mutate whenever you’d like.

    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and happens when these three behaviors occur:

    // Two or more pointers access the same data at the same time.
    // At least one of the pointers is being used to write to the data.
    // There’s no mechanism being used to synchronize access to the data.
    // Data races cause undefined behavior and can be difficult to diagnose and fix when you’re
    // trying to track them down at runtime; Rust prevents this problem from happening because it
    // won’t even compile code with data races!

    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable
    // references, just not simultaneous ones:

    let mut s7 = String::from("hello");

    {
        let r1 = &mut s7;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s7;
    println!("{}", r2);

    // A similar rule exists for combining mutable and immutable references. This code results in
    // an error:

    // let mut s8 = String::from("hello");
    // let r1 = &s8; // no problem
    // let r2 = &s8; // no problem
    // let r3 = &mut s8; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    // Whew! We also cannot have a mutable reference while we have an immutable one. Users of an
    // immutable reference don’t expect the values to suddenly change out from under them! However,
    // multiple immutable references are okay because no one who is just reading the data has the
    // ability to affect anyone else’s reading of the data.

    // Note that a reference’s scope starts from where it is introduced and continues through the
    // last time that reference is used. For instance, this code will compile because the last
    // usage of the immutable references occurs before the mutable reference is introduced:

    let mut s10 = String::from("hello");

    let r1 = &s10; // no problem
    let r2 = &s10; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s10; // no problem
    r3.push_str(" world!!!!!");
    println!("{}", r3);
    // println!("{} {}", r3, r1); // won’t work!

    // The scopes of the immutable references r1 and r2 end after the println! where they are last
    // used, which is before the mutable reference r3 is created. These scopes don’t overlap, so
    // this code is allowed.

    // Even though borrowing errors may be frustrating at times, remember that it’s the Rust
    // compiler pointing out a potential bug early (at compile time rather than at runtime) and
    // showing you exactly where the problem is. Then you don’t have to track down why your data
    // isn’t what you thought it was.

    // =======================================
    // Dangling References
    // =======================================
    // In languages with pointers, it’s easy to erroneously create a dangling pointer, a pointer
    // that references a location in memory that may have been given to someone else, by freeing
    // some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler
    // guarantees that references will never be dangling references: if you have a reference to
    // some data, the compiler will ensure that the data will not go out of scope before the
    // reference to the data does.

    // Let’s try to create a dangling reference, which Rust will prevent with a compile-time error:

    let reference_to_nothing = no_dangle();
    // let reference_to_nothing = dangle();

    // fn dangle() -> &String {
    //     // dangle returns a reference to a String
    //     let s = String::from("hello"); // s is a new String

    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!

    // Because s is created inside dangle, when the code of dangle is finished, s will be
    // deallocated. But we tried to return a reference to it. That means this reference would be
    // pointing to an invalid String. That’s no good! Rust won’t let us do this.

    // The solution here is to return the String directly:
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }

    // =======================================
    // The Rules of References
    // =======================================
    // Let’s recap what we’ve discussed about references:

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.
    // Next, we’ll look at a different kind of reference: slices.
}
