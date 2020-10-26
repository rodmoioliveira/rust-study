// =======================================
// Writing Automated Tests
// =======================================

// In his 1972 essay “The Humble Programmer,” Edsger W. Dijkstra said that “Program testing can be
// a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing
// their absence.” That doesn’t mean we shouldn’t try to test as much as we can!

// Correctness in our programs is the extent to which our code does what we intend it to do. Rust
// is designed with a high degree of concern about the correctness of programs, but correctness is
// complex and not easy to prove. Rust’s type system shoulders a huge part of this burden, but the
// type system cannot catch every kind of incorrectness. As such, Rust includes support for writing
// automated software tests within the language.

// As an example, say we write a function called add_two that adds 2 to whatever number is passed
// to it. This function’s signature accepts an integer as a parameter and returns an integer as a
// result. When we implement and compile that function, Rust does all the type checking and borrow
// checking that you’ve learned so far to ensure that, for instance, we aren’t passing a String
// value or an invalid reference to this function. But Rust can’t check that this function will do
// precisely what we intend, which is return the parameter plus 2 rather than, say, the parameter
// plus 10 or the parameter minus 50! That’s where tests come in.

// We can write tests that assert, for example, that when we pass 3 to the add_two function, the
// returned value is 5. We can run these tests whenever we make changes to our code to make sure
// any existing correct behavior has not changed.

// Testing is a complex skill: although we can’t cover every detail about how to write good tests
// in one chapter, we’ll discuss the mechanics of Rust’s testing facilities. We’ll talk about the
// annotations and macros available to you when writing your tests, the default behavior and
// options provided for running your tests, and how to organize tests into unit tests and
// integration tests.

// =======================================
// How to Write Tests
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
// TODO: continue chapter...
// =======================================

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
