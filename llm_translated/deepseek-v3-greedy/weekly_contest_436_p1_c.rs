use std::cmp::Ordering;
use std::io::{self, Write};

// Comparator for ascending order
fn cmp_asc(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}

// Comparator for descending order
fn cmp_desc(a: &i32, b: &i32) -> Ordering {
    b.cmp(a)
}

fn sort_matrix(grid: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();

    // Sort the diagonals starting from the first row
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

    // Sort the diagonals starting from the first column
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

    grid.to_vec()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input"))
            .collect();
        grid[i] = row;
    }

    let result = sort_matrix(&mut grid);

    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}