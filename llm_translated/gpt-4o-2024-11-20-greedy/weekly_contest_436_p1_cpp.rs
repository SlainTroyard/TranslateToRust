use std::io;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        
        // Sort diagonals starting from the first column
        for i in 0..n {
            let mut vec = Vec::new();
            for k in 0..n - i {
                vec.push(grid[i + k][k]);
            }
            vec.sort_by_key(|&x| Reverse(x)); // Sort in descending order
            for k in 0..n - i {
                grid[i + k][k] = vec[k];
            }
        }
        
        // Sort diagonals starting from the first row
        for j in 1..n {
            let mut vec = Vec::new();
            for k in 0..n - j {
                vec.push(grid[k][j + k]);
            }
            vec.sort(); // Sort in ascending order
            for k in 0..n - j {
                grid[k][j + k] = vec[k];
            }
        }
        
        grid
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut grid = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Process the matrix using the solution
    let solution = Solution;
    let result = solution.sort_matrix(grid);
    
    // Print the result to stdout
    for row in result {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}