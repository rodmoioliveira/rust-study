// =======================================
// Generic Types, Traits, and Lifetimes
// https://doc.rust-lang.org/book/ch10-00-generics.html
// =======================================
// Every programming language has tools for effectively handling the duplication of concepts. In
// Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other
// properties. When we’re writing code, we can express the behavior of generics or how they relate
// to other generics without knowing what will be in their place when compiling and running the
// code.

// Similar to the way a function takes parameters with unknown values to run the same code on
// multiple concrete values, functions can take parameters of some generic type instead of a
// concrete type, like i32 or String. In fact, we’ve already used generics in Chapter 6 with
// Option<T>, Chapter 8 with Vec<T> and HashMap<K, V>, and Chapter 9 with Result<T, E>. In this
// chapter, you’ll explore how to define your own types, functions, and methods with generics!

// First, we’ll review how to extract a function to reduce code duplication. Next, we’ll use the
// same technique to make a generic function from two functions that differ only in the types of
// their parameters. We’ll also explain how to use generic types in struct and enum definitions.

// Then you’ll learn how to use traits to define behavior in a generic way. You can combine traits
// with generic types to constrain a generic type to only those types that have a particular
// behavior, as opposed to just any type.

// Finally, we’ll discuss lifetimes, a variety of generics that give the compiler information about
// how references relate to each other. Lifetimes allow us to borrow values in many situations
// while still enabling the compiler to check that the references are valid.

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// The note mentions std::cmp::PartialOrd, which is a trait. We’ll talk about traits in the next
// section. For now, this error states that the body of largest won’t work for all possible types
// that T could be. Because we want to compare values of type T in the body, we can only use types
// whose values can be ordered. To enable comparisons, the standard library has the
// std::cmp::PartialOrd trait that you can implement on types (see Appendix C for more on this
// trait). You’ll learn how to specify that a generic type has a particular trait in the “Traits as
// Parameters” section, but let’s first explore other ways of using generic type parameters.
fn largest_generic<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    // =======================================
    // Removing Duplication by Extracting a Function
    // =======================================
    // Before diving into generics syntax, let’s first look at how to remove duplication that doesn’t
    // involve generic types by extracting a function. Then we’ll apply this technique to extract a
    // generic function! In the same way that you recognize duplicated code to extract into a function,
    // you’ll start to recognize duplicated code that can use generics.

    // Consider a short program that finds the largest number in a list, as shown in Listing 10-1.

    let number_list = vec![34, 50, 25, 100, 65, 1000];

    // The largest function has a parameter called list, which represents any concrete slice of i32
    // values that we might pass into the function. As a result, when we call the function, the
    // code runs on the specific values that we pass in.

    // In sum, here are the steps we took to change the code from Listing 10-2 to Listing 10-3:

    // Identify duplicate code.  Extract the duplicate code into the body of the function and specify
    // the inputs and return values of that code in the function signature.  Update the two instances
    // of duplicated code to call the function instead.  Next, we’ll use these same steps with generics
    // to reduce code duplication in different ways. In the same way that the function body can operate
    // on an abstract list instead of specific values, generics allow code to operate on abstract
    // types.

    // For example, say we had two functions: one that finds the largest item in a slice of i32 values
    // and one that finds the largest item in a slice of char values. How would we eliminate that
    // duplication? Let’s find out!

    println!("The largest number is {}", largest(&number_list));

    // =======================================
    // Generic Data Types
    // https://doc.rust-lang.org/book/ch10-01-syntax.html
    // =======================================
    // We can use generics to create definitions for items like function signatures or structs,
    // which we can then use with many different concrete data types. Let’s first look at how to
    // define functions, structs, enums, and methods using generics. Then we’ll discuss how
    // generics affect code performance.

    // =======================================
    // In Function Definitions
    // =======================================
    // When defining a function that uses generics, we place the generics in the signature of the
    // function where we would usually specify the data types of the parameters and return value. Doing
    // so makes our code more flexible and provides more functionality to callers of our function while
    // preventing code duplication.

    // Continuing with our largest function, Listing 10-4 shows two functions that both find the
    // largest value in a slice.

    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("The largest number is {}", largest_i32(&number_list));
    println!("The largest char is {}", largest_char(&char_list));
    println!("largest_generic");
    println!("The largest number is {}", largest_generic(&number_list));
    println!("The largest char is {}", largest_generic(&char_list));

    // The largest_i32 function is the one we extracted in Listing 10-3 that finds the largest i32
    // in a slice. The largest_char function finds the largest char in a slice. The function bodies
    // have the same code, so let’s eliminate the duplication by introducing a generic type
    // parameter in a single function.

    // To parameterize the types in the new function we’ll define, we need to name the type parameter,
    // just as we do for the value parameters to a function. You can use any identifier as a type
    // parameter name. But we’ll use T because, by convention, parameter names in Rust are short, often
    // just a letter, and Rust’s type-naming convention is CamelCase. Short for “type,” T is the
    // default choice of most Rust programmers.

    // When we use a parameter in the body of the function, we have to declare the parameter name in
    // the signature so the compiler knows what that name means. Similarly, when we use a type
    // parameter name in a function signature, we have to declare the type parameter name before we use
    // it. To define the generic largest function, place type name declarations inside angle brackets,
    // <>, between the name of the function and the parameter list, like this:

    // fn largest<T>(list: &[T]) -> &T {

    // We read this definition as: the function largest is generic over some type T. This function
    // has one parameter named list, which is a slice of values of type T. The largest function
    // will return a reference to a value of the same type T.

    // Listing 10-5 shows the combined largest function definition using the generic data type in its
    // signature. The listing also shows how we can call the function with either a slice of i32 values
    // or char values. Note that this code won’t compile yet, but we’ll fix it later in this chapter.

    // =======================================
    // In Struct Definitions
    // =======================================

    // We can also define structs to use a generic type parameter in one or more fields using the <>
    // syntax. Listing 10-6 shows how to define a Point<T> struct to hold x and y coordinate values of
    // any type.

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{:?}", integer);
    println!("{:?}", float);

    // The syntax for using generics in struct definitions is similar to that used in function
    // definitions. First, we declare the name of the type parameter inside angle brackets just
    // after the name of the struct. Then we can use the generic type in the struct definition
    // where we would otherwise specify concrete data types.

    // Note that because we’ve used only one generic type to define Point<T>, this definition says that
    // the Point<T> struct is generic over some type T, and the fields x and y are both that same type,
    // whatever that type may be. If we create an instance of a Point<T> that has values of different
    // types, as in Listing 10-7, our code won’t compile.

    let letters = Point { x: 'a', y: 'b' };
    println!("{:?}", letters);

    // let wrong = Point { x: 1, y: 'b' };

    // To define a Point struct where x and y are both generics but could have different types, we
    // can use multiple generic type parameters. For example, in Listing 10-8, we can change the
    // definition of Point to be generic over types T and U where x is of type T and y is of type
    // U.

    #[derive(Debug)]
    struct PointMultipleTypes<T, U> {
        x: T,
        y: U,
    }

    let both_integer = PointMultipleTypes { x: 5, y: 10 };
    let both_float = PointMultipleTypes { x: 1.0, y: 4.0 };
    let integer_and_float = PointMultipleTypes { x: 5, y: 4.0 };
    println!(
        "{:?}\n{:?}\n{:?}",
        both_integer, both_float, integer_and_float
    );

    // Now all the instances of Point shown are allowed! You can use as many generic type
    // parameters in a definition as you want, but using more than a few makes your code hard to
    // read. When you need lots of generic types in your code, it could indicate that your code
    // needs restructuring into smaller pieces.

    // =======================================
    // In Enum Definitions
    // =======================================

    // As we did with structs, we can define enums to hold generic data types in their variants. Let’s
    // take another look at the Option<T> enum that the standard library provides, which we used in
    // Chapter 6:

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    // This definition should now make more sense to you. As you can see, Option<T> is an enum that is
    // generic over type T and has two variants: Some, which holds one value of type T, and a None
    // variant that doesn’t hold any value. By using the Option<T> enum, we can express the abstract
    // concept of having an optional value, and because Option<T> is generic, we can use this
    // abstraction no matter what the type of the optional value is.

    // Enums can use multiple generic types as well. The definition of the Result enum that we used in
    // Chapter 9 is one example:

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a
    // value of type T, and Err, which holds a value of type E. This definition makes it convenient
    // to use the Result enum anywhere we have an operation that might succeed (return a value of
    // some type T) or fail (return an error of some type E). In fact, this is what we used to open
    // a file in Listing 9-3, where T was filled in with the type std::fs::File when the file was
    // opened successfully and E was filled in with the type std::io::Error when there were
    // problems opening the file.

    // When you recognize situations in your code with multiple struct or enum definitions that differ
    // only in the types of the values they hold, you can avoid duplication by using generic types
    // instead.

    // =======================================
    // In Method Definitions
    // =======================================

    // We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in
    // their definitions, too. Listing 10-9 shows the Point<T> struct we defined in Listing 10-6 with a
    // method named x implemented on it.

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // Here, we’ve defined a method named x on Point<T> that returns a reference to the data in the
    // field x.

    // Note that we have to declare T just after impl so we can use it to specify that we’re
    // implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can
    // identify that the type in the angle brackets in Point is a generic type rather than a concrete
    // type.

    // We could, for example, implement methods only on Point<f32> instances rather than on Point<T>
    // instances with any generic type. In Listing 10-10 we use the concrete type f32, meaning we don’t
    // declare any types after impl.

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    println!("{}", float.distance_from_origin());
    // println!("{}", integer.distance_from_origin()); // error!

    // This code means the type Point<f32> will have a method named distance_from_origin and other
    // instances of Point<T> where T is not of type f32 will not have this method defined. The
    // method measures how far our point is from the point at coordinates (0.0, 0.0) and uses
    // mathematical operations that are available only for floating point types.

    // Generic type parameters in a struct definition aren’t always the same as those you use in
    // that struct’s method signatures. For example, Listing 10-11 defines the method mixup on the
    // Point<T, U> struct from Listing 10-8. The method takes another Point as a parameter, which
    // might have different types from the self Point we’re calling mixup on. The method creates a
    // new Point instance with the x value from the self Point (of type T) and the y value from the
    // passed-in Point (of type W).

    impl<T, U> PointMultipleTypes<T, U> {
        fn mixup<V, W>(self, other: PointMultipleTypes<V, W>) -> PointMultipleTypes<T, W> {
            PointMultipleTypes {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = PointMultipleTypes { x: 5, y: 10.4 };
    let p2 = PointMultipleTypes { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // In main, we’ve defined a Point that has an i32 for x (with value 5) and an f64 for y (with
    // value 10.4). The p2 variable is a Point struct that has a string slice for x (with value
    // "Hello") and a char for y (with value c). Calling mixup on p1 with the argument p2 gives us
    // p3, which will have an i32 for x, because x came from p1. The p3 variable will have a char
    // for y, because y came from p2. The println! macro call will print p3.x = 5, p3.y = c.

    // The purpose of this example is to demonstrate a situation in which some generic parameters are
    // declared with impl and some are declared with the method definition. Here, the generic
    // parameters T and U are declared after impl, because they go with the struct definition. The
    // generic parameters V and W are declared after fn mixup, because they’re only relevant to the
    // method.

    // =======================================
    // Performance of Code Using Generics
    // =======================================

    // You might be wondering whether there is a runtime cost when you’re using generic type
    // parameters. The good news is that Rust implements generics in such a way that your code doesn’t
    // run any slower using generic types than it would with concrete types.

    // Rust accomplishes this by performing monomorphization of the code that is using generics at
    // compile time. Monomorphization is the process of turning generic code into specific code by
    // filling in the concrete types that are used when compiled.

    // In this process, the compiler does the opposite of the steps we used to create the generic
    // function in Listing 10-5: the compiler looks at all the places where generic code is called and
    // generates code for the concrete types the generic code is called with.

    // Let’s look at how this works with an example that uses the standard library’s Option<T> enum:

    // let integer = Some(5);
    // let float = Some(5.0);

    // When Rust compiles this code, it performs monomorphization. During that process, the compiler
    // reads the values that have been used in Option<T> instances and identifies two kinds of
    // Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of
    // Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the
    // specific ones.

    // The monomorphized version of the code looks like the following. The generic Option<T> is
    // replaced with the specific definitions created by the compiler:

    // enum Option_i32 {
    //     Some(i32),
    //     None,
    // }

    // enum Option_f64 {
    //     Some(f64),
    //     None,
    // }

    // let integer = Option_i32::Some(5);
    // let float = Option_f64::Some(5.0);

    // Because Rust compiles generic code into code that specifies the type in each instance, we pay no
    // runtime cost for using generics. When the code runs, it performs just as it would if we had
    // duplicated each definition by hand. The process of monomorphization makes Rust’s generics
    // extremely efficient at runtime.
}
