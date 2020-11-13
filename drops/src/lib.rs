// this test code defines a mockmessenger struct that has a sent_messages field with a vec of
// string values to keep track of the messages it’s told to send. we also define an associated
// function new to make it convenient to create new mockmessenger values that start with an empty
// list of messages. we then implement the messenger trait for mockmessenger so we can give a
// mockmessenger to a limittracker. in the definition of the send method, we take the message
// passed in as a parameter and store it in the mockmessenger list of sent_messages.

// In the test, we’re testing what happens when the LimitTracker is told to set value to something
// that is more than 75 percent of the max value. First, we create a new MockMessenger, which will
// start with an empty list of messages. Then we create a new LimitTracker and give it a reference
// to the new MockMessenger and a max value of 100. We call the set_value method on the
// LimitTracker with a value of 80, which is more than 75 percent of 100. Then we assert that the
// list of messages that the MockMessenger is keeping track of should now have one message in it.

// We can’t modify the MockMessenger to keep track of the messages, because the send method takes
// an immutable reference to self. We also can’t take the suggestion from the error text to use
// &mut self instead, because then the signature of send wouldn’t match the signature in the
// Messenger trait definition (feel free to try and see what error message you get).

// This is a situation in which interior mutability can help! We’ll store the sent_messages within
// a RefCell<T>, and then the send message will be able to modify sent_messages to store the
// messages we’ve seen.

// The sent_messages field is now of type RefCell<Vec<String>> instead of Vec<String>. In the new
// function, we create a new RefCell<Vec<String>> instance around the empty vector.

// For the implementation of the send method, the first parameter is still an immutable borrow of
// self, which matches the trait definition. We call borrow_mut on the RefCell<Vec<String>> in
// self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>,
// which is the vector. Then we can call push on the mutable reference to the vector to keep track
// of the messages sent during the test.

// The last change we have to make is in the assertion: to see how many items are in the inner
// vector, we call borrow on the RefCell<Vec<String>> to get an immutable reference to the vector.

// Now that you’ve seen how to use RefCell<T>, let’s dig into how it works!

// =======================================
// Keeping Track of Borrows at Runtime with RefCell<T>
// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#keeping-track-of-borrows-at-runtime-with-refcellt
// =======================================

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
