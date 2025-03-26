fn zigzag_traversal(grid: &[Vec<i32>]) -> Vec<i32> {
    let grid_size = grid.len();
    if grid_size == 0 {
        return Vec::new();
    }
    let grid_col_size = grid[0].len();
    let mut ans = Vec::new();
    let mut c1 = grid_col_size.saturating_sub(1) - (grid_col_size & 1);

    for r in 0..grid_size {
        if r & 1 == 1 { // if r is odd
            let mut c = c1;
            while c >= 0 {
                ans.push(grid[r][c]);
                c -= 2;
                if c < 0 {
                    break;
                }
            }
        } else { // if r is even
            let mut c = 0;
            while c < grid_col_size {
                ans.push(grid[r][c]);
                c += 2;
            }
        }
    }
    ans
}

fn main() {
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut first_line_iter = first_line.split_whitespace();
    let grid_size: usize = first_line_iter.next().unwrap().parse().expect("Invalid input for gridSize");
    let grid_col_size: usize = first_line_iter.next().unwrap().parse().expect("Invalid input for gridColSize");

    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let line = lines.next().unwrap().expect("Failed to read line");
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid input for grid element"))
            .collect();
        grid.push(row);
    }

    let ans = zigzag_traversal(&grid);

    for i in 0..ans.len() {
        write!(writer, "{}", ans[i]).expect("Failed to write");
        if i < ans.len() - 1 {
            write!(writer, " ").expect("Failed to write");
        }
    }
    writeln!(writer).expect("Failed to write newline");
}