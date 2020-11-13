fn main() {
    // =======================================
    // Running Code on Cleanup with the Drop Trait
    // https://doc.rust-lang.org/book/ch15-03-drop.html
    // =======================================
    // The second trait important to the smart pointer pattern is Drop, which lets you customize
    // what happens when a value is about to go out of scope. You can provide an implementation for
    // the Drop trait on any type, and the code you specify can be used to release resources like
    // files or network connections. We’re introducing Drop in the context of smart pointers
    // because the functionality of the Drop trait is almost always used when implementing a smart
    // pointer. For example, when a Box<T> is dropped it will deallocate the space on the heap that
    // the box points to.

    // In some languages, the programmer must call code to free memory or resources every time they
    // finish using an instance of a smart pointer. If they forget, the system might become overloaded
    // and crash. In Rust, you can specify that a particular bit of code be run whenever a value goes
    // out of scope, and the compiler will insert this code automatically. As a result, you don’t need
    // to be careful about placing cleanup code everywhere in a program that an instance of a
    // particular type is finished with—you still won’t leak resources!

    // Specify the code to run when a value goes out of scope by implementing the Drop trait. The Drop
    // trait requires you to implement one method named drop that takes a mutable reference to self. To
    // see when Rust calls drop, let’s implement drop with println! statements for now.

    // Listing 15-14 shows a CustomSmartPointer struct whose only custom functionality is that it will
    // print Dropping CustomSmartPointer! when the instance goes out of scope. This example
    // demonstrates when Rust runs the drop function.

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    // let _c = CustomSmartPointer {
    //     data: "c".to_string(),
    // };
    // let _d = CustomSmartPointer {
    //     data: String::from("d"),
    // };

    // {
    //     println!("{}", _c.data);
    //     println!("{}", _d.data);
    // }

    // println!("CustomSmartPointers created.");

    // The Drop trait is included in the prelude, so we don’t need to bring it into scope. We
    // implement the Drop trait on CustomSmartPointer and provide an implementation for the drop
    // method that calls println!. The body of the drop function is where you would place any logic
    // that you wanted to run when an instance of your type goes out of scope. We’re printing some
    // text here to demonstrate when Rust will call drop.

    // In main, we create two instances of CustomSmartPointer and then print CustomSmartPointers
    // created. At the end of main, our instances of CustomSmartPointer will go out of scope, and Rust
    // will call the code we put in the drop method, printing our final message. Note that we didn’t
    // need to call the drop method explicitly.

    // Rust automatically called drop for us when our instances went out of scope, calling the code
    // we specified. Variables are dropped in the reverse order of their creation, so d was dropped
    // before c. This example gives you a visual guide to how the drop method works; usually you
    // would specify the cleanup code that your type needs to run rather than a print message.

    // =======================================
    // Dropping a Value Early with std::mem::drop
    // =======================================
    // Unfortunately, it’s not straightforward to disable the automatic drop functionality. Disabling
    // drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of
    // automatically. Occasionally, however, you might want to clean up a value early. One example is
    // when using smart pointers that manage locks: you might want to force the drop method that
    // releases the lock so that other code in the same scope can acquire the lock. Rust doesn’t let
    // you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop
    // function provided by the standard library if you want to force a value to be dropped before the
    // end of its scope.

    // If we try to call the Drop trait’s drop method manually by modifying the main function from
    // Listing 15-14, as shown in Listing 15-15, we’ll get a compiler error

    // This error message states that we’re not allowed to explicitly call drop. The error message
    // uses the term destructor, which is the general programming term for a function that cleans
    // up an instance. A destructor is analogous to a constructor, which creates an instance. The
    // drop function in Rust is one particular destructor.

    // Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the
    // value at the end of main. This would be a double free error because Rust would be trying to
    // clean up the same value twice.

    // We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t
    // call the drop method explicitly. So, if we need to force a value to be cleaned up early, we can
    // use the std::mem::drop function.

    // The std::mem::drop function is different from the drop method in the Drop trait. We call it by
    // passing the value we want to force to be dropped early as an argument. The function is in the
    // prelude, so we can modify main in Listing 15-15 to call the drop function, as shown in Listing
    // 15-16:

    let _e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(_e);
    println!("CustomSmartPointer dropped before the end of main.");

    // The text Dropping CustomSmartPointer with data `some data`! is printed between the
    // CustomSmartPointer created. and CustomSmartPointer dropped before the end of main. text,
    // showing that the drop method code is called to drop c at that point.

    // You can use code specified in a Drop trait implementation in many ways to make cleanup
    // convenient and safe: for instance, you could use it to create your own memory allocator! With
    // the Drop trait and Rust’s ownership system, you don’t have to remember to clean up because Rust
    // does it automatically.

    // You also don’t have to worry about problems resulting from accidentally cleaning up values still
    // in use: the ownership system that makes sure references are always valid also ensures that drop
    // gets called only once when the value is no longer being used.

    // Now that we’ve examined Box<T> and some of the characteristics of smart pointers, let’s look at
    // a few other smart pointers defined in the standard library.

    // =======================================
    // Rc<T>, the Reference Counted Smart Pointer
    // https://doc.rust-lang.org/book/ch15-04-rc.html
    // =======================================

    // In the majority of cases, ownership is clear: you know exactly which variable owns a given
    // value. However, there are cases when a single value might have multiple owners. For example,
    // in graph data structures, multiple edges might point to the same node, and that node is
    // conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up
    // unless it doesn’t have any edges pointing to it.

    // To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for
    // reference counting. The Rc<T> type keeps track of the number of references to a value which
    // determines whether or not a value is still in use. If there are zero references to a value, the
    // value can be cleaned up without any references becoming invalid.

    // Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on.
    // Others can come into the room and watch the TV. When the last person leaves the room, they turn
    // off the TV because it’s no longer being used. If someone turns off the TV while others are still
    // watching it, there would be uproar from the remaining TV watchers!

    // We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our
    // program to read and we can’t determine at compile time which part will finish using the data
    // last. If we knew which part would finish last, we could just make that part the data’s owner,
    // and the normal ownership rules enforced at compile time would take effect.

    // Note that Rc<T> is only for use in single-threaded scenarios. When we discuss concurrency in
    // Chapter 16, we’ll cover how to do reference counting in multithreaded programs.

    // =======================================
    // Using Rc<T> to Share Data
    // =======================================

    // Let’s return to our cons list example in Listing 15-5. Recall that we defined it using Box<T>.
    // This time, we’ll create two lists that both share ownership of a third list. Conceptually, this
    // looks similar to Figure 15-3:

    // https://doc.rust-lang.org/book/img/trpl15-03.svg

    // enum List {
    //     Cons(i32, Box<List>),
    //     Nil,
    // }

    // let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    // let b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a));

    // The Cons variants own the data they hold, so when we create the b list, a is moved into b
    // and b owns a. Then, when we try to use a again when creating c, we’re not allowed to because
    // a has been moved.

    // We could change the definition of Cons to hold references instead, but then we would have to
    // specify lifetime parameters. By specifying lifetime parameters, we would be specifying that
    // every element in the list will live at least as long as the entire list. The borrow checker
    // wouldn’t let us compile let a = Cons(10, &Nil); for example, because the temporary Nil value
    // would be dropped before a could take a reference to it.

    // Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>, as shown in
    // Listing 15-18. Each Cons variant will now hold a value and an Rc<T> pointing to a List. When we
    // create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is holding, thereby
    // increasing the number of references from one to two and letting a and b share ownership of the
    // data in that Rc<List>. We’ll also clone a when creating c, increasing the number of references
    // from two to three. Every time we call Rc::clone, the reference count to the data within the
    // Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.

    use std::rc::Rc;

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(4, Rc::clone(&a));

    // We need to add a use statement to bring Rc<T> into scope because it’s not in the prelude. In
    // main, we create the list holding 5 and 10 and store it in a new Rc<List> in a. Then when we
    // create b and c, we call the Rc::clone function and pass a reference to the Rc<List> in a as
    // an argument.

    // We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use
    // Rc::clone in this case. The implementation of Rc::clone doesn’t make a deep copy of all the data
    // like most types’ implementations of clone do. The call to Rc::clone only increments the
    // reference count, which doesn’t take much time. Deep copies of data can take a lot of time. By
    // using Rc::clone for reference counting, we can visually distinguish between the deep-copy kinds
    // of clones and the kinds of clones that increase the reference count. When looking for
    // performance problems in the code, we only need to consider the deep-copy clones and can
    // disregard calls to Rc::clone.

    // =======================================
    // Cloning an Rc<T> Increases the Reference Count
    // =======================================
    // Let’s change our working example in Listing 15-18 so we can see the reference counts changing as
    // we create and drop references to the Rc<List> in a.

    // In Listing 15-19, we’ll change main so it has an inner scope around list c; then we can see how
    // the reference count changes when c goes out of scope.

    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // At each point in the program where the reference count changes, we print the reference
    // count, which we can get by calling the Rc::strong_count function. This function is named
    // strong_count rather than count because the Rc<T> type also has a weak_count; we’ll see what
    // weak_count is used for in the “Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>”
    // section.

    // We can see that the Rc<List> in a has an initial reference count of 1; then each time we
    // call clone, the count goes up by 1. When c goes out of scope, the count goes down by 1. We
    // don’t have to call a function to decrease the reference count like we have to call Rc::clone
    // to increase the reference count: the implementation of the Drop trait decreases the
    // reference count automatically when an Rc<T> value goes out of scope.

    // What we can’t see in this example is that when b and then a go out of scope at the end of main,
    // the count is then 0, and the Rc<List> is cleaned up completely at that point. Using Rc<T> allows
    // a single value to have multiple owners, and the count ensures that the value remains valid as
    // long as any of the owners still exist.

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program
    // for reading only. If Rc<T> allowed you to have multiple mutable references too, you might
    // violate one of the borrowing rules discussed in Chapter 4: multiple mutable borrows to the same
    // place can cause data races and inconsistencies. But being able to mutate data is very useful! In
    // the next section, we’ll discuss the interior mutability pattern and the RefCell<T> type that you
    // can use in conjunction with an Rc<T> to work with this immutability restriction.

    // =======================================
    // RefCell<T> and the Interior Mutability Pattern
    // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    // =======================================
}
