use std::io::{self, BufRead};

fn max_sum(grid: &mut Vec<Vec<i32>>, grid_size: usize, grid_col_size: &usize, limits: &[i32], limits_size: usize, k: i32) -> i64 {
    let mut len = 0;
    for i in 0..limits_size {
        len += limits[i] as usize;
    }
    
    let mut lst = vec![0; len];
    let mut l = 0;
    
    for i in 0..grid_size {
        // Sort each row in descending order
        grid[i].sort_by(|a, b| b.cmp(a));
        
        for j in 0..limits[i] as usize {
            lst[l] = grid[i][j];
            l += 1;
        }
    }
    
    // Sort the combined list in descending order
    lst.sort_by(|a, b| b.cmp(a));
    
    // Calculate the sum of the k largest elements
    let mut ans: i64 = 0;
    for i in 0..k as usize {
        ans += lst[i] as i64;
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n, m, k
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Read limits
    let line = lines.next().unwrap().unwrap();
    let limits: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate and print the result
    let result = max_sum(&mut grid, n, &m, &limits, n, k);
    println!("{}", result);
}