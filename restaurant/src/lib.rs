// =======================================
// Defining Modules to Control Scope and Privacy
// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// =======================================

// In this section, we’ll talk about modules and other parts of the module system, namely paths
// that allow you to name items; the use keyword that brings a path into scope; and the pub keyword
// to make items public. We’ll also discuss the as keyword, external packages, and the glob
// operator. For now, let’s focus on modules!

// Modules let us organize code within a crate into groups for readability and easy reuse. Modules
// also control the privacy of items, which is whether an item can be used by outside code (public)
// or is an internal implementation detail and not available for outside use (private).

// As an example, let’s write a library crate that provides the functionality of a restaurant.
// We’ll define the signatures of functions but leave their bodies empty to concentrate on the
// organization of the code, rather than actually implement a restaurant in code.

// In the restaurant industry, some parts of a restaurant are referred to as front of house and
// others as back of house. Front of house is where customers are; this is where hosts seat
// customers, servers take orders and payment, and bartenders make drinks. Back of house is where
// the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative
// work.

// To structure our crate in the same way that a real restaurant works, we can organize the
// functions into nested modules. Create a new library named restaurant by running cargo new --lib
// restaurant; then put the code in Listing 7-1 into src/lib.rs to define some modules and function
// signatures.

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        pub fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    pub mod serving {
        fn hide() {
            println!("private!!!");
        }

        pub fn take_order() {
            println!("take_order");
        }

        pub fn serve_order() {
            println!("serve_order");
            crate::front_of_house::serving::hide();
            super::serving::hide();
        }

        pub fn take_payment() {
            println!("take_payment");
        }
    }
}

// We define a module by starting with the mod keyword and then specify the name of the module (in
// this case, front_of_house) and place curly brackets around the body of the module. Inside
// modules, we can have other modules, as in this case with the modules hosting and serving.
// Modules can also hold definitions for other items, such as structs, enums, constants, traits,
// or—as in Listing 7-1—functions.

// By using modules, we can group related definitions together and name why they’re related.
// Programmers using this code would have an easier time finding the definitions they wanted to use
// because they could navigate the code based on the groups rather than having to read through all
// the definitions. Programmers adding new functionality to this code would know where to place the
// code to keep the program organized.

// Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots. The reason for
// their name is that the contents of either of these two files form a module named crate at the
// root of the crate’s module structure, known as the module tree.

// Listing 7-2 shows the module tree for the structure in Listing 7-1.

// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// This tree shows how some of the modules nest inside one another (for example, hosting nests
// inside front_of_house). The tree also shows that some modules are siblings to each other,
// meaning they’re defined in the same module (hosting and serving are defined within
// front_of_house). To continue the family metaphor, if module A is contained inside module B, we
// say that module A is the child of module B and that module B is the parent of module A. Notice
// that the entire module tree is rooted under the implicit module named crate.

// The module tree might remind you of the filesystem’s directory tree on your computer; this is a
// very apt comparison! Just like directories in a filesystem, you use modules to organize your
// code. And just like files in a directory, we need a way to find our modules.

// =======================================
// Paths for Referring to an Item in the Module Tree
// =======================================
// To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

// A path can take two forms:

// An absolute path starts from a crate root by using a crate name or a literal crate.
// A relative path starts from the current module and uses self, super, or an identifier in the current module.
// Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

// Let’s return to the example in Listing 7-1. How do we call the add_to_waitlist function? This is the same as asking, what’s the path of the add_to_waitlist function? In Listing 7-3, we simplified our code a bit by removing some of the modules and functions. We’ll show two ways to call the add_to_waitlist function from a new function eat_at_restaurant defined in the crate root. The eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword. In the ”Exposing Paths with the pub Keyword” section, we’ll go into more detail about pub. Note that this example won’t compile just yet; we’ll explain why in a bit.

// The first time we call the add_to_waitlist function in eat_at_restaurant, we use an absolute
// path. The add_to_waitlist function is defined in the same crate as eat_at_restaurant, which
// means we can use the crate keyword to start an absolute path.

// After crate, we include each of the successive modules until we make our way to add_to_waitlist.
// You can imagine a filesystem with the same structure, and we’d specify the path
// /front_of_house/hosting/add_to_waitlist to run the add_to_waitlist program; using the crate name
// to start from the crate root is like using / to start from the filesystem root in your shell.

// The second time we call add_to_waitlist in eat_at_restaurant, we use a relative path. The path
// starts with front_of_house, the name of the module defined at the same level of the module tree
// as eat_at_restaurant. Here the filesystem equivalent would be using the path
// front_of_house/hosting/add_to_waitlist. Starting with a name means that the path is relative.

// Choosing whether to use a relative or absolute path is a decision you’ll make based on your
// project. The decision should depend on whether you’re more likely to move item definition code
// separately from or together with the code that uses the item. For example, if we move the
// front_of_house module and the eat_at_restaurant function into a module named
// customer_experience, we’d need to update the absolute path to add_to_waitlist, but the relative
// path would still be valid. However, if we moved the eat_at_restaurant function separately into a
// module named dining, the absolute path to the add_to_waitlist call would stay the same, but the
// relative path would need to be updated. Our preference is to specify absolute paths because it’s
// more likely to move code definitions and item calls independently of each other.

// The error messages say that module hosting is private. In other words, we have the correct paths
// for the hosting module and the add_to_waitlist function, but Rust won’t let us use them because
// it doesn’t have access to the private sections.

// Modules aren’t useful only for organizing your code. They also define Rust’s privacy boundary:
// the line that encapsulates the implementation details external code isn’t allowed to know about,
// call, or rely on. So, if you want to make an item like a function or struct private, you put it
// in a module.

// The way privacy works in Rust is that all items (functions, methods, structs, enums, modules,
// and constants) are private by default. Items in a parent module can’t use the private items
// inside child modules, but items in child modules can use the items in their ancestor modules.
// The reason is that child modules wrap and hide their implementation details, but the child
// modules can see the context in which they’re defined. To continue with the restaurant metaphor,
// think of the privacy rules as being like the back office of a restaurant: what goes on in there
// is private to restaurant customers, but office managers can see and do everything in the
// restaurant in which they operate.

// Rust chose to have the module system function this way so that hiding inner implementation
// details is the default. That way, you know which parts of the inner code you can change without
// breaking outer code. But you can expose inner parts of child modules' code to outer ancestor
// modules by using the pub keyword to make an item public.

// Exposing Paths with the pub Keyword Let’s return to the error in Listing 7-4 that told us the
// hosting module is private. We want the eat_at_restaurant function in the parent module to have
// access to the add_to_waitlist function in the child module, so we mark the hosting module with
// the pub keyword, as shown in Listing 7-5.

// What happened? Adding the pub keyword in front of mod hosting makes the module public. With this
// change, if we can access front_of_house, we can access hosting. But the contents of hosting are
// still private; making the module public doesn’t make its contents public. The pub keyword on a
// module only lets code in its ancestor modules refer to it.

// The errors in Listing 7-6 say that the add_to_waitlist function is private. The privacy rules
// apply to structs, enums, functions, and methods as well as modules.

// Let’s also make the add_to_waitlist function public by adding the pub keyword before its
// definition, as in Listing 7-7.

// Now the code will compile! Let’s look at the absolute and the relative path and double-check why
// adding the pub keyword lets us use these paths in add_to_waitlist with respect to the privacy
// rules.

// In the absolute path, we start with crate, the root of our crate’s module tree. Then the
// front_of_house module is defined in the crate root. The front_of_house module isn’t public, but
// because the eat_at_restaurant function is defined in the same module as front_of_house (that is,
// eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from
// eat_at_restaurant. Next is the hosting module marked with pub. We can access the parent module
// of hosting, so we can access hosting. Finally, the add_to_waitlist function is marked with pub
// and we can access its parent module, so this function call works!

// In the relative path, the logic is the same as the absolute path except for the first step:
// rather than starting from the crate root, the path starts from front_of_house. The
// front_of_house module is defined within the same module as eat_at_restaurant, so the relative
// path starting from the module in which eat_at_restaurant is defined works. Then, because hosting
// and add_to_waitlist are marked with pub, the rest of the path works, and this function call is
// valid!

// =======================================
// Starting Relative Paths with super
// https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#starting-relative-paths-with-super
// =======================================

// We can also construct relative paths that begin in the parent module by using super at the start
// of the path. This is like starting a filesystem path with the .. syntax. Why would we want to do
// this?

// Consider the code in Listing 7-8 that models the situation in which a chef fixes an incorrect
// order and personally brings it out to the customer. The function fix_incorrect_order calls the
// function serve_order by specifying the path to serve_order starting with super:

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// The fix_incorrect_order function is in the back_of_house module, so we can use super to go to
// the parent module of back_of_house, which in this case is crate, the root. From there, we look
// for serve_order and find it. Success! We think the back_of_house module and the serve_order
// function are likely to stay in the same relationship to each other and get moved together should
// we decide to reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer
// places to update code in the future if this code gets moved to a different module.

// =======================================
// Making Structs and Enums Public
// =======================================
// We can also use pub to designate structs and enums as public, but there are a few extra details.
// If we use pub before a struct definition, we make the struct public, but the struct’s fields
// will still be private. We can make each field public or not on a case-by-case basis. In Listing
// 7-9, we’ve defined a public back_of_house::Breakfast struct with a public toast field but a
// private seasonal_fruit field. This models the case in a restaurant where the customer can pick
// the type of bread that comes with a meal, but the chef decides which fruit accompanies the meal
// based on what’s in season and in stock. The available fruit changes quickly, so customers can’t
// choose the fruit or even see which fruit they’ll get.

pub mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Because the toast field in the back_of_house::Breakfast struct is public, in eat_at_restaurant
    // we can write and read to the toast field using dot notation. Notice that we can’t use the
    // seasonal_fruit field in eat_at_restaurant because seasonal_fruit is private. Try uncommenting
    // the line modifying the seasonal_fruit field value to see what error you get!

    // Also, note that because back_of_house::Breakfast has a private field, the struct needs to
    // provide a public associated function that constructs an instance of Breakfast (we’ve named it
    // summer here). If Breakfast didn’t have such a function, we couldn’t create an instance of
    // Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit
    // field in eat_at_restaurant.

    // In contrast, if we make an enum public, all of its variants are then public. We only need the
    // pub before the enum keyword, as shown in Listing 7-10.

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Because we made the Appetizer enum public, we can use the Soup and Salad variants in
    // eat_at_restaurant. Enums aren’t very useful unless their variants are public; it would be
    // annoying to have to annotate all enum variants with pub in every case, so the default for
    // enum variants is to be public. Structs are often useful without their fields being public,
    // so struct fields follow the general rule of everything being private by default unless
    // annotated with pub.

    // There’s one more situation involving pub that we haven’t covered, and that is our last module
    // system feature: the use keyword. We’ll cover use by itself first, and then we’ll show how to
    // combine pub and use.
}

// =======================================
// Bringing Paths into Scope with the use Keyword
// =======================================
// It might seem like the paths we’ve written to call functions so far are inconveniently long and
// repetitive. For example, in Listing 7-7, whether we chose the absolute or relative path to the
// add_to_waitlist function, every time we wanted to call add_to_waitlist we had to specify
// front_of_house and hosting too. Fortunately, there’s a way to simplify this process. We can
// bring a path into a scope once and then call the items in that path as if they’re local items
// with the use keyword.

// In Listing 7-11, we bring the crate::front_of_house::hosting module into the scope of the
// eat_at_restaurant function so we only have to specify hosting::add_to_waitlist to call the
// add_to_waitlist function in eat_at_restaurant.
use self::front_of_house::hosting;

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. By
// adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that
// scope, just as though the hosting module had been defined in the crate root. Paths brought into
// scope with use also check privacy, like any other paths.

// You can also bring an item into scope with use and a relative path. Listing 7-12 shows how to
// specify a relative path to get the same behavior as in Listing 7-11.
use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn pay_restaurant() {
    serving::take_payment();
    serving::take_payment();
    serving::take_payment();
}

// =======================================
// Creating Idiomatic use Paths
// https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
// =======================================
