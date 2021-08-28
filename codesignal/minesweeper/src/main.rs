// =======================================
// Problem
// https://app.codesignal.com/arcade/intro/level-5/ZMR5n7vJbexnLrgaM
// =======================================
// In the popular Minesweeper game you have a board with some mines and those cells that don't
// contain a mine have a number in it that indicates the total number of mines in the neighboring
// cells. Starting off with some arrangement of mines we want to create a Minesweeper game setup.

// Example

// For

// matrix = [[true, false, false],
//           [false, true, false],
//           [false, false, false]]

// the output should be

// minesweeper(matrix) = [[1, 2, 1],
//                        [2, 1, 1],
//                        [1, 1, 1]]
// Check out the image below for better understanding:

// Input/Output

// [execution time limit] 2 seconds (rs)

// [input] array.array.boolean matrix

// A non-empty rectangular matrix consisting of boolean values - true if the corresponding cell
// contains a mine, false otherwise.

// Guaranteed constraints:
// 2 ≤ matrix.length ≤ 100,
// 2 ≤ matrix[0].length ≤ 100.

// [output] array.array.integer

// Rectangular matrix of the same size as matrix each cell of which contains an integer equal to
// the number of mines in the neighboring cells. Two cells are called neighboring if they share at
// least one corner.

fn minesweeper(matrix: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    vec![vec![]]
}

fn main() {
    let tests: Vec<(Vec<Vec<bool>>, Vec<Vec<i32>>)> = vec![
        (
            vec![
                vec![true, false, false],
                vec![false, true, false],
                vec![false, false, false],
            ],
            vec![vec![1, 2, 1], vec![2, 1, 1], vec![1, 1, 1]],
        ),
        (
            vec![
                vec![true, false, false, true],
                vec![false, false, true, false],
                vec![true, true, false, true],
            ],
            vec![vec![0, 2, 2, 1], vec![3, 4, 3, 3], vec![1, 2, 3, 1]],
        ),
    ];

    tests.iter().for_each(|(input, expect)| {
        let result = minesweeper(input.clone());
        assert_eq!(
            result,
            expect.clone(),
            "expect that result {:?} = {:?} for {:?}",
            result,
            expect,
            input
        );
    });
}
