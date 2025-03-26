use std::io::{self, BufRead};

/// Compare function for sorting in descending order
fn max_sum(grid: &mut Vec<Vec<i32>>, _grid_col_size: i32, limits: &[i32], limits_size: i32, k: i32) -> i64 {
    // Calculate the total length of elements we need to collect
    let mut len = 0;
    for i in 0..limits_size as usize {
        len += limits[i] as usize;
    }
    
    // Collect top elements from each row based on limits
    let mut lst = Vec::with_capacity(len);
    let mut l = 0;
    for i in 0..grid.len() {
        // Sort the row in descending order
        grid[i].sort_by(|a, b| b.cmp(a));
        
        // Take the first limits[i] elements from the row
        for j in 0..limits[i] as usize {
            lst.push(grid[i][j]);
            l += 1;
        }
    }
    
    // Sort the collected elements in descending order
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
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    
    // Allocate and read the grid
    let mut grid = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Read the limits
    let mut limits = Vec::with_capacity(n as usize);
    let line = lines.next().unwrap().unwrap();
    for limit in line.split_whitespace() {
        limits.push(limit.parse::<i32>().unwrap());
    }
    
    // Compute and print the result
    println!("{}", max_sum(&mut grid, m, &limits, n, k));
}