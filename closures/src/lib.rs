// =======================================
// Limitations of the Cacher Implementation
// =======================================
// Caching values is a generally useful behavior that we might want to use in other parts of our
// code with different closures. However, there are two problems with the current implementation of
// Cacher that would make reusing it in different contexts difficult.

// The first problem is that a Cacher instance assumes it will always get the same value for the
// parameter arg to the value method. That is, this test of Cacher will fail:

// #[test]
// fn call_with_different_values() {
//     let mut c = Cacher::new(|a| a);

//     let v1 = c.value(1);
//     let v2 = c.value(2);

//     assert_eq!(v2, 2);
// }
