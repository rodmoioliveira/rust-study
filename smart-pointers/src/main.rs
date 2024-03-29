// =======================================
// Smart Pointers
// https://doc.rust-lang.org/book/ch15-00-smart-pointers.html#smart-pointers
// =======================================
// A pointer is a general concept for a variable that contains an address in memory. This address
// refers to, or “points at,” some other data. The most common kind of pointer in Rust is a
// reference, which you learned about in Chapter 4. References are indicated by the & symbol and
// borrow the value they point to. They don’t have any special capabilities other than referring to
// data. Also, they don’t have any overhead and are the kind of pointer we use most often.

// Smart pointers, on the other hand, are data structures that not only act like a pointer but also
// have additional metadata and capabilities. The concept of smart pointers isn’t unique to Rust:
// smart pointers originated in C++ and exist in other languages as well. In Rust, the different
// smart pointers defined in the standard library provide functionality beyond that provided by
// references. One example that we’ll explore in this chapter is the reference counting smart
// pointer type. This pointer enables you to have multiple owners of data by keeping track of the
// number of owners and, when no owners remain, cleaning up the data.

// In Rust, which uses the concept of ownership and borrowing, an additional difference between
// references and smart pointers is that references are pointers that only borrow data; in
// contrast, in many cases, smart pointers own the data they point to.

// We’ve already encountered a few smart pointers in this book, such as String and Vec<T> in
// Chapter 8, although we didn’t call them smart pointers at the time. Both these types count as
// smart pointers because they own some memory and allow you to manipulate it. They also have
// metadata (such as their capacity) and extra capabilities or guarantees (such as with String
// ensuring its data will always be valid UTF-8).

// Smart pointers are usually implemented using structs. The characteristic that distinguishes a
// smart pointer from an ordinary struct is that smart pointers implement the Deref and Drop
// traits. The Deref trait allows an instance of the smart pointer struct to behave like a
// reference so you can write code that works with either references or smart pointers. The Drop
// trait allows you to customize the code that is run when an instance of the smart pointer goes
// out of scope. In this chapter, we’ll discuss both traits and demonstrate why they’re important
// to smart pointers.

// Given that the smart pointer pattern is a general design pattern used frequently in Rust, this
// chapter won’t cover every existing smart pointer. Many libraries have their own smart pointers,
// and you can even write your own. We’ll cover the most common smart pointers in the standard
// library:

// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at
// runtime instead of compile time

// In addition, we’ll cover the interior mutability pattern where an immutable type exposes an API
// for mutating an interior value. We’ll also discuss reference cycles: how they can leak memory
// and how to prevent them.

// Let’s dive in!

// =======================================
// Using Box<T> to Point to Data on the Heap
// =======================================
// The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you
// to store data on the heap rather than the stack. What remains on the stack is the pointer to the
// heap data. Refer to Chapter 4 to review the difference between the stack and the heap.

// Boxes don’t have performance overhead, other than storing their data on the heap instead of on
// the stack. But they don’t have many extra capabilities either. You’ll use them most often in
// these situations:

// When you have a type whose size can’t be known at compile time and you want to use a value of
// that type in a context that requires an exact size

// When you have a large amount of data and you want to transfer ownership but ensure the data
// won’t be copied when you do so

// When you want to own a value and you care only that it’s a type that implements a particular
// trait rather than being of a specific type

// We’ll demonstrate the first situation in the “Enabling Recursive Types with Boxes” section. In
// the second case, transferring ownership of a large amount of data can take a long time because
// the data is copied around on the stack. To improve performance in this situation, we can store
// the large amount of data on the heap in a box. Then, only the small amount of pointer data is
// copied around on the stack, while the data it references stays in one place on the heap. The
// third case is known as a trait object, and Chapter 17 devotes an entire section, “Using Trait
// Objects That Allow for Values of Different Types,” just to that topic. So what you learn here
// you’ll apply again in Chapter 17!

fn main() {
    // =======================================
    // Using a Box<T> to Store Data on the Heap
    // =======================================
    // Before we discuss this use case for Box<T>, we’ll cover the syntax and how to interact with
    // values stored within a Box<T>.

    // We define the variable b to have the value of a Box that points to the value 5, which is
    // allocated on the heap. This program will print b = 5; in this case, we can access the data
    // in the box similar to how we would if this data were on the stack. Just like any owned
    // value, when a box goes out of scope, as b does at the end of main, it will be deallocated.
    // The deallocation happens for the box (stored on the stack) and the data it points to (stored
    // on the heap).

    // Putting a single value on the heap isn’t very useful, so you won’t use boxes by themselves in
    // this way very often. Having values like a single i32 on the stack, where they’re stored by
    // default, is more appropriate in the majority of situations. Let’s look at a case where boxes
    // allow us to define types that we wouldn’t be allowed to if we didn’t have boxes.

    let b = Box::new(5);
    println!("b = {}", b);

    // =======================================
    // Enabling Recursive Types with Boxes
    // =======================================
    // At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be
    // known at compile time is a recursive type, where a value can have as part of itself another
    // value of the same type. Because this nesting of values could theoretically continue infinitely,
    // Rust doesn’t know how much space a value of a recursive type needs. However, boxes have a known
    // size, so by inserting a box in a recursive type definition, you can have recursive types.

    // Let’s explore the cons list, which is a data type common in functional programming languages, as
    // an example of a recursive type. The cons list type we’ll define is straightforward except for
    // the recursion; therefore, the concepts in the example we’ll work with will be useful any time
    // you get into more complex situations involving recursive types.

    // =======================================
    // More Information About the Cons List
    // =======================================
    // A cons list is a data structure that comes from the Lisp programming language and its
    // dialects. In Lisp, the cons function (short for “construct function”) constructs a new pair
    // from its two arguments, which usually are a single value and another pair. These pairs
    // containing pairs form a list.

    // The cons function concept has made its way into more general functional programming jargon: “to
    // cons x onto y” informally means to construct a new container instance by putting the element x
    // at the start of this new container, followed by the container y.

    // Each item in a cons list contains two elements: the value of the current item and the next item.
    // The last item in the list contains only a value called Nil without a next item. A cons list is
    // produced by recursively calling the cons function. The canonical name to denote the base case of
    // the recursion is Nil. Note that this is not the same as the “null” or “nil” concept in Chapter
    // 6, which is an invalid or absent value.

    // Although functional programming languages use cons lists frequently, the cons list isn’t a
    // commonly used data structure in Rust. Most of the time when you have a list of items in Rust,
    // Vec<T> is a better choice to use. Other, more complex recursive data types are useful in various
    // situations, but by starting with the cons list, we can explore how boxes let us define a
    // recursive data type without much distraction.

    // Listing 15-2 contains an enum definition for a cons list. Note that this code won’t compile yet
    // because the List type doesn’t have a known size, which we’ll demonstrate.

    // Note: We’re implementing a cons list that holds only i32 values for the purposes of this
    // example. We could have implemented it using generics, as we discussed in Chapter 10, to
    // define a cons list type that could store values of any type.

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // Using the List type to store the list 1, 2, 3 would look like the code in Listing 15-3:

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);

    // The first Cons value holds 1 and another List value. This List value is another Cons value
    // that holds 2 and another List value. This List value is one more Cons value that holds 3 and
    // a List value, which is finally Nil, the non-recursive variant that signals the end of the
    // list.

    // The error shows this type “has infinite size.” The reason is that we’ve defined List with a
    // variant that is recursive: it holds another value of itself directly. As a result, Rust
    // can’t figure out how much space it needs to store a List value. Let’s break down why we get
    // this error a bit. First, let’s look at how Rust decides how much space it needs to store a
    // value of a non-recursive type.

    // =======================================
    // Computing the Size of a Non-Recursive Type
    // =======================================
    // Recall the Message enum we defined in Listing 6-2 when we discussed enum definitions in Chapter
    // 6:

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // To determine how much space to allocate for a Message value, Rust goes through each of the
    // variants to see which variant needs the most space. Rust sees that Message::Quit doesn’t
    // need any space, Message::Move needs enough space to store two i32 values, and so forth.
    // Because only one variant will be used, the most space a Message value will need is the space
    // it would take to store the largest of its variants.

    // Contrast this with what happens when Rust tries to determine how much space a recursive type
    // like the List enum in Listing 15-2 needs. The compiler starts by looking at the Cons variant,
    // which holds a value of type i32 and a value of type List. Therefore, Cons needs an amount of
    // space equal to the size of an i32 plus the size of a List. To figure out how much memory the
    // List type needs, the compiler looks at the variants, starting with the Cons variant. The Cons
    // variant holds a value of type i32 and a value of type List, and this process continues
    // infinitely, as shown in Figure 15-1.

    // =======================================
    // Using Box<T> to Get a Recursive Type with a Known Size
    // https://doc.rust-lang.org/book/ch15-01-box.html#using-boxt-to-get-a-recursive-type-with-a-known-size
    // =======================================
    // Rust can’t figure out how much space to allocate for recursively defined types, so the
    // compiler gives the error in Listing 15-4. But the error does include this helpful
    // suggestion:

    // = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable

    // In this suggestion, “indirection” means that instead of storing a value directly, we’ll change
    // the data structure to store the value indirectly by storing a pointer to the value instead.

    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s
    // size doesn’t change based on the amount of data it’s pointing to. This means we can put a
    // Box<T> inside the Cons variant instead of another List value directly. The Box<T> will point
    // to the next List value that will be on the heap rather than inside the Cons variant.
    // Conceptually, we still have a list, created with lists “holding” other lists, but this
    // implementation is now more like placing the items next to one another rather than inside one
    // another.

    // We can change the definition of the List enum in Listing 15-2 and the usage of the List in
    // Listing 15-3 to the code in Listing 15-5, which will compile:

    // The Cons variant will need the size of an i32 plus the space to store the box’s pointer
    // data. The Nil variant stores no values, so it needs less space than the Cons variant. We now
    // know that any List value will take up the size of an i32 plus the size of a box’s pointer
    // data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure
    // out the size it needs to store a List value.

    // Boxes provide only the indirection and heap allocation; they don’t have any other special
    // capabilities, like those we’ll see with the other smart pointer types. They also don’t have
    // any performance overhead that these special capabilities incur, so they can be useful in
    // cases like the cons list where the indirection is the only feature we need. We’ll look at
    // more use cases for boxes in Chapter 17, too.

    // The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T>
    // values to be treated like references. When a Box<T> value goes out of scope, the heap data that
    // the box is pointing to is cleaned up as well because of the Drop trait implementation. Let’s
    // explore these two traits in more detail. These two traits will be even more important to the
    // functionality provided by the other smart pointer types we’ll discuss in the rest of this
    // chapter.
}
