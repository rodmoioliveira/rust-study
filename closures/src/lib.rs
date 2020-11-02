// =======================================
// Limitations of the Cacher Implementation
// =======================================
// Caching values is a generally useful behavior that we might want to use in other parts of our
// code with different closures. However, there are two problems with the current implementation of
// Cacher that would make reusing it in different contexts difficult.

// The first problem is that a Cacher instance assumes it will always get the same value for the
// parameter arg to the value method. That is, this test of Cacher will fail:

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            // memoization...
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// This test creates a new Cacher instance with a closure that returns the value passed into it. We
// call the value method on this Cacher instance with an arg value of 1 and then an arg value of 2,
// and we expect the call to value with the arg value of 2 to return 2.

// The problem is that the first time we called c.value with 1, the Cacher instance saved Some(1)
// in self.value. Thereafter, no matter what we pass in to the value method, it will always return
// 1.

// Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map
// will be the arg values that are passed in, and the values of the hash map will be the result of
// calling the closure on that key. Instead of looking at whether self.value directly has a Some or
// a None value, the value function will look up the arg in the hash map and return the value if
// it’s present. If it’s not present, the Cacher will call the closure and save the resulting value
// in the hash map associated with its arg value.

// The second problem with the current Cacher implementation is that it only accepts closures that
// take one parameter of type u32 and return a u32. We might want to cache the results of closures
// that take a string slice and return usize values, for example. To fix this issue, try
// introducing more generic parameters to increase the flexibility of the Cacher functionality.

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
