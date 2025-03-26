use std::cmp::Ordering;
use std::io::{self, Write};

// Comparator function for ascending order
fn cmp_asc(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}

// Comparator function for descending order
fn cmp_desc(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

// Function to sort the matrix diagonally
fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();

    // Sort diagonals starting from the first row
    for i in 0..n {
        let len = n - i;
        let mut a = Vec::with_capacity(len);
        for k in i..n {
            a.push(grid[k][k - i]);
        }
        a.sort_by(cmp_desc);
        for k in i..n {
            grid[k][k - i] = a[k - i];
        }
    }

    // Sort diagonals starting from the first column
    for i in 1..n {
        let len = n - i;
        let mut a = Vec::with_capacity(len);
        for k in 0..n - i {
            a.push(grid[k][k + i]);
        }
        a.sort_by(cmp_asc);
        for k in 0..n - i {
            grid[k][k + i] = a[k];
        }
    }
}

fn main() {
    // Read the size of the matrix
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    // Read the matrix
    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        grid[i] = row;
    }

    // Sort the matrix diagonally
    sort_matrix(&mut grid);

    // Print the sorted matrix
    for i in 0..n {
        for j in 0..n {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}