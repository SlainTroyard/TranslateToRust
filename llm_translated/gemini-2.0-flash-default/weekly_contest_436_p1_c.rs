use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let grid_size = grid.len();

    // Sort diagonals from top-left to bottom-right in descending order
    for i in 0..grid_size {
        let len = grid_size - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        let mut index = 0;
        for (k, m) in (i..grid_size).zip(0..grid_size) {
            a.push(grid[k][m]);
            index += 1;
        }

        a.sort_by(|x, y| y.cmp(x));

        index = 0;
        for (k, m) in (i..grid_size).zip(0..grid_size) {
            grid[k][m] = a[index];
            index += 1;
        }
    }

    // Sort diagonals from top-right to bottom-left in ascending order
    for i in 1..grid_size {
        let len = grid_size - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        let mut index = 0;
        for (k, m) in (0..grid_size).zip(i..grid_size) {
            a.push(grid[k][m]);
            index += 1;
        }

        a.sort();

        index = 0;
        for (k, m) in (0..grid_size).zip(i..grid_size) {
            grid[k][m] = a[index];
            index += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let n: usize = iterator
        .next()
        .unwrap()
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Failed to parse n");

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = iterator
            .next()
            .unwrap()
            .expect("Failed to read line")
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse integer"))
            .collect();
        grid.push(row);
    }

    sort_matrix(&mut grid);

    for i in 0..n {
        for j in 0..n {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}