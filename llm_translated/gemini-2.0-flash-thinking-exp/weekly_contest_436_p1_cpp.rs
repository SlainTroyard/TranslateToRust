fn main() {
    use std::io;

    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row_str = String::new();
        io::stdin().read_line(&mut row_str).expect("Failed to read line");
        let row: Vec<i32> = row_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for grid element"))
            .collect();
        grid.push(row);
    }

    let solution = Solution {};
    let result = solution.sort_matrix(grid);

    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}

struct Solution {}

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        for i in 0..n {
            let mut vec: Vec<i32> = Vec::new();
            for k in 0..(n - i) {
                vec.push(grid[i + k][k]);
            }
            vec.sort_by(|a, b| b.cmp(a)); // Sort in descending order
            for k in 0..(n - i) {
                grid[i + k][k] = vec[k];
            }
        }
        for j in 1..n {
            let mut vec: Vec<i32> = Vec::new();
            for k in 0..(n - j) {
                vec.push(grid[k][j + k]);
            }
            vec.sort(); // Sort in ascending order (default)
            for k in 0..(n - j) {
                grid[k][j + k] = vec[k];
            }
        }
        grid
    }
}