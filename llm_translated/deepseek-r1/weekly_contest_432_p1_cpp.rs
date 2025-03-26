use std::io::{self, Read};

fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    
    for (i, row) in grid.iter().enumerate() {
        let mut cnt = 0;
        if i % 2 == 0 {
            // Even row: iterate left to right, collect every other element starting from first
            for j in 0..row.len() {
                if cnt == 0 {
                    result.push(row[j]);
                }
                cnt ^= 1;
            }
        } else {
            // Odd row: iterate right to left, collect every other element starting from last
            for j in (0..row.len()).rev() {
                if cnt == 0 {
                    result.push(row[j]);
                }
                cnt ^= 1;
            }
        }
    }
    
    result
}

fn main() {
    // Read all input at once and split into tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().expect("Invalid input"));
    
    // Parse grid dimensions
    let n = tokens.next().expect("Missing grid rows") as usize;
    let m = tokens.next().expect("Missing grid columns") as usize;
    
    // Collect grid elements and verify count
    let elements: Vec<i32> = tokens.collect();
    assert!(elements.len() == n * m, "Incorrect number of grid elements");
    
    // Build grid from elements
    let grid: Vec<Vec<i32>> = elements.chunks_exact(m).map(|chunk| chunk.to_vec()).collect();
    
    // Perform traversal and print result
    let result = zigzag_traversal(grid);
    println!("{}", result.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
}