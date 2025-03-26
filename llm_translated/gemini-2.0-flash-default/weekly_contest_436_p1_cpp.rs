use std::io;
use std::io::Read;

struct Solution {}

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<int>>) -> Vec<Vec<int>> {
        let n = grid.len();
        for i in 0..n {
            let mut vec: Vec<int> = Vec::new();
            for k in 0..(n - i) {
                vec.push(grid[i + k][k]);
            }
            vec.sort_by(|a, b| b.cmp(a));
            for k in 0..(n - i) {
                grid[i + k][k] = vec[k];
            }
        }
        for j in 1..n {
            let mut vec: Vec<int> = Vec::new();
            for k in 0..(n - j) {
                vec.push(grid[k][j + k]);
            }
            vec.sort();
            for k in 0..(n - j) {
                grid[k][j + k] = vec[k];
            }
        }
        grid
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut grid: Vec<Vec<int>> = Vec::new();
    for _ in 0..n {
        let row: Vec<int> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<int>().unwrap())
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