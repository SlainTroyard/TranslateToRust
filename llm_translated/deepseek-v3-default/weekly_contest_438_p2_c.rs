use std::cmp::Ordering;
use std::io;

// Comparator function to sort integers in descending order
fn cmp(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

// Function to calculate the maximum sum of the top k elements from the grid
fn max_sum(grid: &Vec<Vec<i32>>, limits: &Vec<usize>, k: usize) -> i64 {
    let mut lst = Vec::new();
    
    // Iterate over each row in the grid
    for (i, row) in grid.iter().enumerate() {
        let mut sorted_row = row.clone();
        sorted_row.sort_by(cmp); // Sort the row in descending order
        lst.extend(sorted_row.iter().take(limits[i]).cloned()); // Take the top `limits[i]` elements
    }
    
    lst.sort_by(cmp); // Sort the combined list in descending order
    lst.iter().take(k).map(|&x| x as i64).sum() // Sum the top k elements
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid input");
    let m: usize = parts.next().unwrap().parse().expect("Invalid input");
    let k: usize = parts.next().unwrap().parse().expect("Invalid input");
    
    let mut grid = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        grid.push(row);
    }
    
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let limits: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();
    
    let result = max_sum(&grid, &limits, k);
    println!("{}", result);
}