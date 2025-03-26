use std::io::{self, BufRead, Write};

fn zigzag_traversal(grid: &Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let m = grid[0].len();
    let mut vec = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        if i & 1 == 1 {
            // Traverse the row from right to left
            for j in (0..m).rev() {
                vec.push(row[j]);
            }
        } else {
            // Traverse the row from left to right
            for &val in row.iter() {
                vec.push(val);
            }
        }
    }

    vec
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();

    // Read the size of the grid
    stdin_lock.read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid = vec![vec![0; m]; n];

    // Read the grid values
    for i in 0..n {
        input.clear();
        stdin_lock.read_line(&mut input)?;
        let mut iter = input.split_whitespace();
        for j in 0..m {
            grid[i][j] = iter.next().unwrap().parse().unwrap();
        }
    }

    // Perform the zigzag traversal
    let result = zigzag_traversal(&grid);

    // Output the result
    for (i, &val) in result.iter().enumerate() {
        write!(stdout, "{}{}", val, if i < result.len() - 1 { " " } else { "" })?;
    }

    Ok(())
}