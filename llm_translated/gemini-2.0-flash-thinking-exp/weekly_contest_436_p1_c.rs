use std::io::{self, BufRead, Write};
use std::vec;

fn sort_matrix(grid: &mut Vec<Vec<i32>>) {
    let n = grid.len();
    for i in 0..n {
        let len = n - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        for k in 0..len {
            a.push(grid[i + k][k]);
        }
        a.sort_by(|x, y| y.cmp(x)); // Descending sort
        for k in 0..len {
            grid[i + k][k] = a[k];
        }
    }
    for i in 1..n {
        let len = n - i;
        let mut a: Vec<i32> = Vec::with_capacity(len);
        for k in 0..len {
            a.push(grid[k][i + k]);
        }
        a.sort(); // Ascending sort (default)
        for k in 0..len {
            grid[k][i + k] = a[k];
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout);

    let mut n_str = String::new();
    reader.read_line(&mut n_str)?;
    let n: usize = n_str.trim().parse().unwrap();

    let mut grid: Vec<Vec<i32>> = vec![vec![0; n]; n];
    for i in 0..n {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let nums: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..n {
            grid[i][j] = nums[j];
        }
    }

    sort_matrix(&mut grid);

    for i in 0..n {
        for j in 0..n {
            write!(writer, "{} ", grid[i][j])?;
        }
        writeln!(writer)?;
    }
    writer.flush()?;
    Ok(())
}