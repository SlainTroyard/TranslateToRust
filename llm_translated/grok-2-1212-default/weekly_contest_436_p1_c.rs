use std::io::{self, BufRead};

fn sort_matrix(grid: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();

    // Sort diagonals from bottom-left to top-right in descending order
    for i in 0..n {
        let len = n - i;
        let mut diagonal = Vec::with_capacity(len);
        for k in i..n {
            diagonal.push(grid[k][k - i]);
        }
        diagonal.sort_by(|a, b| b.cmp(a));
        for k in i..n {
            grid[k][k - i] = diagonal[k - i];
        }
    }

    // Sort diagonals from top-left to bottom-right in ascending order
    for i in 1..n {
        let len = n - i;
        let mut diagonal = Vec::with_capacity(len);
        for k in 0..n - i {
            diagonal.push(grid[k][k + i]);
        }
        diagonal.sort();
        for k in 0..n - i {
            grid[k][k + i] = diagonal[k];
        }
    }

    grid.clone()
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the size of the grid
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Read the grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = lines.next().unwrap()?
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }

    // Sort the matrix
    let result = sort_matrix(&mut grid);

    // Print the result
    for row in result {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }

    Ok(())
}