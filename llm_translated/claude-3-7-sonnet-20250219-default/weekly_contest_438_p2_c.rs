use std::io::{self, BufRead};

fn max_sum(grid: &mut [Vec<i32>], limits: &[i32], k: i32) -> i64 {
    let mut len = 0;
    for &limit in limits {
        len += limit;
    }
    
    let mut lst = Vec::with_capacity(len as usize);
    
    for (i, row) in grid.iter_mut().enumerate() {
        // Sort row in descending order
        row.sort_unstable_by(|a, b| b.cmp(a));
        
        // Take the top 'limits[i]' elements from each row
        for j in 0..limits[i] as usize {
            lst.push(row[j]);
        }
    }
    
    // Sort all collected elements in descending order
    lst.sort_unstable_by(|a, b| b.cmp(a));
    
    // Take the sum of the k largest elements
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
    let params: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = params[0] as usize;
    let m = params[1] as usize;
    let k = params[2];
    
    // Read grid
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    
    // Read limits
    let mut limits = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        limits.push(line.trim().parse::<i32>().unwrap());
    }
    
    // Compute and print result
    println!("{}", max_sum(&mut grid, &limits, k));
}