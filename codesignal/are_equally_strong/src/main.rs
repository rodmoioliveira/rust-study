// =======================================
// Problem
// https://app.codesignal.com/arcade/intro/level-5/g6dc9KJyxmFjB98dL
// =======================================

// Call two arms equally strong if the heaviest weights they each are able to lift are equal.

// Call two people equally strong if their strongest arms are equally strong (the strongest arm can
// be both the right and the left), and so are their weakest arms.

// Given your and your friend's arms' lifting capabilities find out if you two are equally strong.

// Example

// For yourLeft = 10, yourRight = 15, friendsLeft = 15, and friendsRight = 10, the output should be
// areEquallyStrong(yourLeft, yourRight, friendsLeft, friendsRight) = true;
// For yourLeft = 15, yourRight = 10, friendsLeft = 15, and friendsRight = 10, the output should be
// areEquallyStrong(yourLeft, yourRight, friendsLeft, friendsRight) = true;
// For yourLeft = 15, yourRight = 10, friendsLeft = 15, and friendsRight = 9, the output should be
// areEquallyStrong(yourLeft, yourRight, friendsLeft, friendsRight) = false.

fn are_equally_strong(
    your_left: i32,
    your_right: i32,
    friends_left: i32,
    friends_right: i32,
) -> bool {
    let mut you = vec![your_right, your_left];
    let mut friend = vec![friends_right, friends_left];
    you.sort();
    friend.sort();
    you == friend
}

fn main() {
    println!("{}", are_equally_strong(15, 10, 15, 9));
}
