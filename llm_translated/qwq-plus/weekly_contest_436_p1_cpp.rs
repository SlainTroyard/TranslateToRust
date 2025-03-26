use std::io;

struct Solution;

impl Solution {
    pub fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
        let n = grid.len();
        for i in 0..n {
            let mut vec = Vec::new();
            let count = n - i;
            for k in 0..count {
                let row = i + k;
                let col = k;
                vec.push(grid[row][col]);
            }
            vec.sort_by(|a, b| b.cmp(a)); // Sort in descending order
            for (k, &val) in vec.iter().enumerate() {
                let row = i + k;
                let col = k;
                grid[row][col] = val;
            }
        }
        for j in 1..n {
            let mut vec = Vec::new();
            let count = n - j;
            for k in 0..count {
                let row = k;
                let col = j + k;
                vec.push(grid[row][col]);
            }
            vec.sort(); // Sort in ascending order
            for (k, &val) in vec.iter().enumerate() {
                let row = k;
                let col = j + k;
                grid[row][col] = val;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap() as usize;
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = iter.next().unwrap();
        }
    }
    Solution::sort_matrix(&mut grid);
    for row in &grid {
        for &num in row {
            print!("{} ", num);
        }
        println!();
    }
}