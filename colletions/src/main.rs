// =======================================
// Common Collections
// =======================================
// Rust’s standard library includes a number of very useful data structures called collections.
// Most other data types represent one specific value, but collections can contain multiple values.
// Unlike the built-in array and tuple types, the data these collections point to is stored on the
// heap, which means the amount of data does not need to be known at compile time and can grow or
// shrink as the program runs. Each kind of collection has different capabilities and costs, and
// choosing an appropriate one for your current situation is a skill you’ll develop over time. In
// this chapter, we’ll discuss three collections that are used very often in Rust programs:

// * A vector allows you to store a variable number of values next to each other.
// * A string is a collection of characters. We’ve mentioned the String type previously, but in
// this chapter we’ll talk about it in depth.
// * A hash map allows you to associate a value with a particular key. It’s a particular
// implementation of the more general data structure called a map.
// * To learn about the other kinds of collections provided by the standard library, see the documentation.

// We’ll discuss how to create and update vectors, strings, and hash maps, as well as what makes
// each special.

fn main() {
    // =======================================
    // Storing Lists of Values with Vectors
    // https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors
    // =======================================
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("{:?}", v1);
    println!("{:?}", v2);

    {
        let v3 = vec![1];
        println!("{:?}", v3);
    }
    // cannot find value `v3` in this scope
    // println!("{:?}", v3);

    // =======================================
    // Reading Elements of Vectors
    // =======================================
    let x: Vec<i32> = vec![0; 5];
    let third: &i32 = &x[2];
    println!("{:?}", third);

    match x.get(20) {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element."),
    }

    let does_not_exist = x.get(100);
    println!("{:?}", does_not_exist);

    // why should a reference to the first element care about what changes at the end of the
    // vector? This error is due to the way vectors work: adding a new element onto the end of the
    // vector might require allocating new memory and copying the old elements to the new space, if
    // there isn’t enough room to put all the elements next to each other where the vector
    // currently is. In that case, the reference to the first element would be pointing to
    // deallocated memory. The borrowing rules prevent programs from ending up in that situation.

    // let mut z = vec![1, 2, 3, 4, 5];
    // let first = &z[0];
    // z.push(6);
    // println!("The first element is: {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value that the mutable reference refers to, we have to use the dereference
        // operator (*) to get to the value in i before we can use the += operator. We’ll talk more
        // about the dereference operator in the “Following the Pointer to the Value with the
        // Dereference Operator” section of Chapter 15.
        *i += 50;
    }
    println!("{:?}", v);

    // =======================================
    // Using an Enum to Store Multiple Types
    // =======================================

    // let no_multiple_types = vec![1, "a"];

    // At the beginning of this chapter, we said that vectors can only store values that are the same
    // type. This can be inconvenient; there are definitely use cases for needing to store a list of
    // items of different types. Fortunately, the variants of an enum are defined under the same enum
    // type, so when we need to store elements of a different type in a vector, we can define and use
    // an enum!

    // For example, say we want to get values from a row in a spreadsheet in which some of the columns
    // in the row contain integers, some floating-point numbers, and some strings. We can define an
    // enum whose variants will hold the different value types, and then all the enum variants will be
    // considered the same type: that of the enum. Then we can create a vector that holds that enum and
    // so, ultimately, holds different types. We’ve demonstrated this in Listing 8-10.

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    // Rust needs to know what types will be in the vector at compile time so it knows exactly how
    // much memory on the heap will be needed to store each element. A secondary advantage is that
    // we can be explicit about what types are allowed in this vector. If Rust allowed a vector to
    // hold any type, there would be a chance that one or more of the types would cause errors with
    // the operations performed on the elements of the vector. Using an enum plus a match
    // expression means that Rust will ensure at compile time that every possible case is handled,
    // as discussed in Chapter 6.

    // When you’re writing a program, if you don’t know the exhaustive set of types the program will
    // get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait
    // object, which we’ll cover in Chapter 17.

    // Now that we’ve discussed some of the most common ways to use vectors, be sure to review the API
    // documentation for all the many useful methods defined on Vec<T> by the standard library. For
    // example, in addition to push, a pop method removes and returns the last element. Let’s move on
    // to the next collection type: String!
}
