use std::collections::BinaryHeap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().map(|s| s.trim()).filter(|s| !s.is_empty());
    
    // Read the first line containing n, m, k
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    // Read the grid
    let mut grid = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();
        grid[i] = nums;
    }

    // Read the limits
    let limits_line = lines.next().unwrap();
    let limits: Vec<usize> = limits_line.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();

    // Sort each row in descending order
    for row in &mut grid {
        row.sort_by(|a, b| b.cmp(a));
    }

    // Initialize the max-heap
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((grid[i][0], i, 0));
    }

    let mut ans = 0;
    while k > 0 && !heap.is_empty() {
        let (val, r, c) = heap.pop().unwrap();
        if c >= limits[r] {
            continue;
        }
        ans += val;
        k -= 1;
        if c + 1 < m {
            heap.push((grid[r][c + 1], r, c + 1));
        }
    }

    println!("{}", ans);
}