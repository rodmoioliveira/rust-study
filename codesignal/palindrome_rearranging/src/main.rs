// =======================================
// Problem
// https://app.codesignal.com/arcade/intro/level-4/Xfeo7r9SBSpo3Wico
// =======================================
// Given a string, find out if its characters can be rearranged to form a palindrome.

// Example

// For inputString = "aabb", the output should be
// palindromeRearranging(inputString) = true.

// We can rearrange "aabb" to make "abba", which is a palindrome.

// Input/Output

// [execution time limit] 2 seconds (rs)

// [input] string inputString

// A string consisting of lowercase English letters.

// Guaranteed constraints:
// 1 ≤ inputString.length ≤ 50.

// [output] boolean

// true if the characters of the inputString can be rearranged to form a palindrome, false otherwise.

use std::collections::HashMap;

// =======================================
// My solution
// =======================================
fn palindrome_rearranging(input_string: String) -> bool {
    let mut table: HashMap<char, u64> = HashMap::new();
    for c in input_string.chars() {
        let count = table.entry(c).or_insert(0);
        *count += 1;
    }

    table
        .into_iter()
        .map(|(_, v)| v)
        .filter(|v| v % 2 != 0)
        .collect::<Vec<u64>>()
        .len()
        < 2
}

// =======================================
// Top Rust solution
// =======================================
fn _palindrome_rearranging(input_string: String) -> bool {
    let mut table: HashMap<char, u64> = HashMap::new();
    input_string
        .chars()
        .for_each(|c| *table.entry(c).or_insert(0) += 1);
    table.values().filter(|v| *v % 2 != 0).count() < 2
}

fn main() {
    // {'c': 4, 'a': 43, 'b': 3}
    // [4, 43, 3]
    // false
    let input = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbccccaaaaaaaaaaaaa".to_string();

    // {'z': 1, 'a': 2}
    // [1, 2]
    // true
    // let input = "zaa";

    // {'c': 1, 'b': 4, 'a': 2}
    // [1, 4, 2]
    // true
    // let input = "abbcabb";

    println!("{:?}", palindrome_rearranging(input));
}
