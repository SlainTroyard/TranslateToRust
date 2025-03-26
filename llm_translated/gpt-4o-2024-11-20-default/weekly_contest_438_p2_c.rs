use std::io;
use std::cmp::Reverse;

// Translate the algorithm to idiomatic Rust
fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<usize>, k: usize) -> i64 {
    let mut all_values = Vec::new();

    // Sort each row and collect the top `limits[i]` elements into `all_values`
    for (row, &limit) in grid.iter().zip(limits.iter()) {
        let mut sorted_row = row.clone();
        sorted_row.sort_unstable_by_key(|&x| Reverse(x));
        all_values.extend(sorted_row.into_iter().take(limit));
    }

    // Sort all collected values in descending order
    all_values.sort_unstable_by_key(|&x| Reverse(x));

    // Calculate the sum of the top `k` elements
    all_values.iter().take(k).map(|&x| x as i64).sum()
}

fn main() {
    let mut input = String::new();
    
    // Read the first line containing n, m, k
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut iter = input.trim().split_whitespace().map(|x| x.parse::<usize>().expect("Invalid number"));
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();
    let k = iter.next().unwrap();

    let mut grid = Vec::new();

    // Read the grid values (n rows, each of m columns)
    for _ in 0..n {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Invalid number"))
            .collect::<Vec<i32>>();
        grid.push(row);
    }

    // Read the limits array
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let limits = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("Invalid number"))
        .collect::<Vec<usize>>();

    // Call the main function and output the result
    let result = max_sum(grid, limits, k);
    println!("{}", result);
}