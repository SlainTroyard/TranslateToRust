use std::io;
use std::io::Read;

fn sort_matrix(mut grid: Vec<Vec<int>>) -> Vec<Vec<int>> {
    let n = grid.len();

    // Process diagonals starting from the top row
    for i in 0..n {
        let mut vec = Vec::new();
        for k in 0..(n - i) {
            vec.push(grid[i + k][k]);
        }
        vec.sort_by(|a, b| b.cmp(a)); // Sort in descending order

        for k in 0..(n - i) {
            grid[i + k][k] = vec[k];
        }
    }

    // Process diagonals starting from the left column (excluding the main diagonal)
    for j in 1..n {
        let mut vec = Vec::new();
        for k in 0..(n - j) {
            vec.push(grid[k][j + k]);
        }
        vec.sort(); // Sort in ascending order

        for k in 0..(n - j) {
            grid[k][j + k] = vec[k];
        }
    }

    grid
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n = lines.next().unwrap().parse::<usize>().unwrap();

    let mut grid: Vec<Vec<int>> = Vec::new();
    for _ in 0..n {
        let row: Vec<int> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<int>().unwrap())
            .collect();
        grid.push(row);
    }

    let result = sort_matrix(grid);

    for i in 0..n {
        for j in 0..n {
            print!("{} ", result[i][j]);
        }
        println!();
    }

    Ok(())
}