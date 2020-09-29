// =======================================
// Method Syntax
// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
// =======================================
// Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.

// =======================================
// Defining Methods
// =======================================
// Let’s change the area function that has a Rectangle instance as a parameter and instead make an area method defined on the Rectangle struct, as shown in Listing 5-13.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("Hello, ola!");

    let a = Rectangle {
        width: 10,
        height: 20,
    };

    println!("area of Rectangle is {}", a.area());
}
