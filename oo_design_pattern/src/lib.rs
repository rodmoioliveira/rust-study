// =======================================
// Defining Post and Creating a New Instance in the Draft State
// =======================================
// Let’s get started on the implementation of the library! We know we need a public Post struct
// that holds some content, so we’ll start with the definition of the struct and an associated
// public new function to create an instance of Post, as shown in Listing 17-12. We’ll also make a
// private State trait. Then Post will hold a trait object of Box<dyn State> inside an Option<T> in
// a private field named state. You’ll see why the Option<T> is necessary in a bit.

// The State trait defines the behavior shared by different post states, and the Draft,
// PendingReview, and Published states will all implement the State trait. For now, the trait
// doesn’t have any methods, and we’ll start by defining just the Draft state because that is the
// state we want a post to start in.

// When we create a new Post, we set its state field to a Some value that holds a Box. This Box
// points to a new instance of the Draft struct. This ensures whenever we create a new instance of
// Post, it will start out as a draft. Because the state field of Post is private, there is no way
// to create a Post in any other state! In the Post::new function, we set the content field to a
// new, empty String.

// =======================================
// Storing the Text of the Post Content
// =======================================
// Listing 17-11 showed that we want to be able to call a method named add_text and pass it a &str
// that is then added to the text content of the blog post. We implement this as a method rather
// than exposing the content field as pub. This means we can implement a method later that will
// control how the content field’s data is read. The add_text method is pretty straightforward, so
// let’s add the implementation in Listing 17-13 to the impl Post block:

// The add_text method takes a mutable reference to self, because we’re changing the Post instance
// that we’re calling add_text on. We then call push_str on the String in content and pass the text
// argument to add to the saved content. This behavior doesn’t depend on the state the post is in,
// so it’s not part of the state pattern. The add_text method doesn’t interact with the state field
// at all, but it is part of the behavior we want to support.

// =======================================
// Ensuring the Content of a Draft Post Is Empty
// =======================================

// Even after we’ve called add_text and added some content to our post, we still want the content
// method to return an empty string slice because the post is still in the draft state, as shown on
// line 7 of Listing 17-11. For now, let’s implement the content method with the simplest thing
// that will fulfill this requirement: always returning an empty string slice. We’ll change this
// later once we implement the ability to change a post’s state so it can be published. So far,
// posts can only be in the draft state, so the post content should always be empty. Listing 17-14
// shows this placeholder implementation:

// =======================================
// Requesting a Review of the Post Changes Its State
// =======================================

// Next, we need to add functionality to request a review of a post, which should change its state
// from Draft to PendingReview. Listing 17-15 shows this code:

// We give Post a public method named request_review that will take a mutable reference to self.
// Then we call an internal request_review method on the current state of Post, and this second
// request_review method consumes the current state and returns a new state.

// We’ve added the request_review method to the State trait; all types that implement the trait
// will now need to implement the request_review method. Note that rather than having self, &self,
// or &mut self as the first parameter of the method, we have self: Box<Self>. This syntax means
// the method is only valid when called on a Box holding the type. This syntax takes ownership of
// Box<Self>, invalidating the old state so the state value of the Post can transform into a new
// state.

// To consume the old state, the request_review method needs to take ownership of the state value.
// This is where the Option in the state field of Post comes in: we call the take method to take
// the Some value out of the state field and leave a None in its place, because Rust doesn’t let us
// have unpopulated fields in structs. This lets us move the state value out of Post rather than
// borrowing it. Then we’ll set the post’s state value to the result of this operation.

// We need to set state to None temporarily rather than setting it directly with code like
// self.state = self.state.request_review(); to get ownership of the state value. This ensures Post
// can’t use the old state value after we’ve transformed it into a new state.

// The request_review method on Draft needs to return a new, boxed instance of a new PendingReview
// struct, which represents the state when a post is waiting for a review. The PendingReview struct
// also implements the request_review method but doesn’t do any transformations. Rather, it returns
// itself, because when we request a review on a post already in the PendingReview state, it should
// stay in the PendingReview state.

// Now we can start seeing the advantages of the state pattern: the request_review method on Post
// is the same no matter its state value. Each state is responsible for its own rules.

// We’ll leave the content method on Post as is, returning an empty string slice. We can now have a
// Post in the PendingReview state as well as in the Draft state, but we want the same behavior in
// the PendingReview state. Listing 17-11 now works up to line 10!

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
