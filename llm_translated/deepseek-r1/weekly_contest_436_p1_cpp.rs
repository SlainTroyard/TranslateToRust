use std::io::{self, Read};

fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();
    // Process diagonals starting from the first column (sorted in descending order)
    for i in 0..n {
        let mut diagonal = Vec::new();
        let mut k = 0;
        while i + k < n {
            diagonal.push(grid[i + k][k]);
            k += 1;
        }
        diagonal.sort_by(|a, b| b.cmp(a));
        for (idx, &val) in diagonal.iter().enumerate() {
            grid[i + idx][idx] = val;
        }
    }
    // Process diagonals starting from the first row (j >= 1, sorted in ascending order)
    for j in 1..n {
        let mut diagonal = Vec::new();
        let mut k = 0;
        while j + k < n {
            diagonal.push(grid[k][j + k]);
            k += 1;
        }
        diagonal.sort();
        for (idx, &val) in diagonal.iter().enumerate() {
            grid[idx][j + idx] = val;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    
    // Read matrix size
    let n: usize = tokens.next().unwrap().parse().unwrap();
    
    // Read matrix elements
    let numbers: Vec<i32> = tokens
        .by_ref()
        .take(n * n)
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Build matrix from flat vector
    let mut grid: Vec<Vec<i32>> = numbers
        .chunks_exact(n)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    // Process matrix
    sort_matrix(&mut grid);
    
    // Print result
    for row in grid {
        println!(
            "{}",
            row.iter()
                .map(|num| num.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}