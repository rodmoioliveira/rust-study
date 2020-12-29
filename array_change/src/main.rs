// =======================================
// Problem
// https://app.codesignal.com/arcade/intro/level-4/xvkRbxYkdHdHNCKjg/drafts
// =======================================
// You are given an array of integers. On each move you are allowed to increase exactly one of its
// element by one. Find the minimal number of moves required to obtain a strictly increasing
// sequence from the input.

// Example

// For inputArray = [1, 1, 1], the output should be arrayChange(inputArray) = 3.

// Input/Output

// [execution time limit] 2 seconds (rs)

// [input] array.integer inputArray

// Guaranteed constraints:
// 3 ≤ inputArray.length ≤ 105, -105 ≤ inputArray[i] ≤ 105.

// [output] integer

// The minimal number of moves needed to obtain a strictly increasing sequence from inputArray.
// It's guaranteed that for the given test cases the answer always fits signed 32-bit integer type.

fn array_change(input_array: Vec<i32>) -> i32 {
    let mut v: Vec<i32> = vec![];
    let mut moves = 0;

    for (i, el) in input_array.iter().enumerate() {
        v.push(*el);

        if i != 0 {
            let cur = v[i];
            let prev = v[i - 1];
            let diff: i32 = cur - prev;

            if diff <= 0 {
                let diff_abs = diff.abs();
                let s = &mut v[i];
                *s = *s + diff_abs + 1;
                moves = moves + diff_abs + 1;
            }
        }
    }

    moves
}

fn main() {
    let a = vec![3122, -645, 2616, 13213, -8069];
    println!("{}", array_change(a));
}
