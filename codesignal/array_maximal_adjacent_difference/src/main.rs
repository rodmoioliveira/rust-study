// =======================================
// Problem
// https://app.codesignal.com/arcade/intro/level-5/EEJxjQ7oo7C5wAGjE
// =======================================

// Given an array of integers, find the maximal absolute difference between any two of its adjacent elements.

// Example

// For inputArray = [2, 4, 1, 0], the output should be
// arrayMaximalAdjacentDifference(inputArray) = 3.

// Input/Output

// [execution time limit] 2 seconds (rs)

// [input] array.integer inputArray

// Guaranteed constraints:
// 3 ≤ inputArray.length ≤ 10,
// -15 ≤ inputArray[i] ≤ 15.

// [output] integer

// The maximal absolute difference.

fn array_maximal_adjancent_difference(input_array: Vec<i32>) -> i32 {
    (1..input_array.len())
        .map(|index| (input_array[index] - input_array[index - 1]).abs())
        .reduce(i32::max)
        .unwrap()
}

fn main() {
    let tests: Vec<(Vec<i32>, i32)> = vec![
        (vec![2, 4, 1, 0], 3),
        (vec![1, 1, 1, 1], 0),
        (vec![-1, 4, 10, 3, -2], 7),
        (vec![10, 11, 13], 2),
        (vec![-2, -2, -2, -2, -2], 0),
        (vec![-1, 1, -3, -4], 4),
        (vec![-14, 15, -15], 30),
    ];

    tests.iter().for_each(|(input, expect)| {
        let result = array_maximal_adjancent_difference(input.to_vec());
        assert_eq!(
            result, *expect,
            "expect that result {} = expect {}",
            result, *expect
        );
    });
}
