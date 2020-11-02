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

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
