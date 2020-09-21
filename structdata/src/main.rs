// =======================================
// Defining and Instantiating Structs
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// =======================================
// Structs are similar to tuples, which were discussed in Chapter 3. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

// To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. For example, Listing 5-1 shows a struct that stores information about a user account.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields. We don’t have to specify the fields in the same order in which we declared them in the struct. In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    // To get a specific value from a struct, we can use dot notation. If we wanted just this user’s email address, we could use user1.email wherever we wanted to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Listing 5-3 shows how to change the value in the email field of a mutable User instance.
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    println!("{}", user2.email);

    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

    // Listing 5-4 shows a build_user function that returns a User instance with the given email and username. The active field gets the value of true, and the sign_in_count gets a value of 1.
    // Here, we’re creating a new instance of the User struct, which has a field named email. We want to set the email field’s value to the value in the email parameter of the build_user function. Because the email field and the email parameter have the same name, we only need to write email rather than email: email.
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let email = String::from("rodmoi.olivera@gmail.com");
    let username = String::from("rodmoi");
    let new_user = build_user(email, username);
    println!("{}", new_user.email);
    // =======================================
    // Creating Instances From Other Instances With Struct Update Syntax
    // =======================================
    // It’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some. You’ll do this using struct update syntax.
    // Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    let another_user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        ..new_user
    };

    println!("{}", another_user.active);
    println!("{}", another_user.username);
    println!("{}", another_user.sign_in_count);

    // =======================================
    // Using Tuple Structs without Named Fields to Create Different Types
    // =======================================
    // You can also define structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.
    // To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple. For example, here are definitions and usages of two tuple structs named Color and Point:

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn create_color(a: i32, b: i32, c: i32) -> Color {
        Color(a, b, c)
    }
    fn create_point(a: i32, b: i32, c: i32) -> Point {
        Point(a, b, c)
    }

    let black = create_color(10, 20, 30);
    let origin = create_point(40, 50, 60);

    println!("{}", black.1);
    println!("{}", origin.2);

    // Note that the black and origin values are different types, because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces, you can use a . followed by the index to access an individual value, and so on.

    // =======================================
    // Unit-Like Structs Without Any Fields
    // =======================================
    // You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself. We’ll discuss traits in Chapter 10.

    // =======================================
    // Ownership of Struct Data
    // =======================================
    // In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

    // It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:

    // struct User2 {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs, but for now, we’ll fix errors like these using owned types like String instead of references like &str.
}
