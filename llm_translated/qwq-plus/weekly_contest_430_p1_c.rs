use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let grid_size = numbers[0] as usize;
    let cols = numbers[1] as usize;

    // Check that there are enough elements
    let total_elements_needed = 2 + grid_size * cols;
    if numbers.len() < total_elements_needed {
        panic!("Input does not contain enough elements");
    }

    let mut grid = Vec::with_capacity(grid_size);
    let mut elements = &numbers[2..];
    for _ in 0..grid_size {
        let row = elements[..cols].to_vec();
        grid.push(row);
        elements = &elements[cols..];
    }

    let result = minimum_operations(&grid);
    println!("{}", result);
}

/// Computes the minimum operations required to make each element strictly greater than the one above it in its column.
///
/// # Arguments
/// * `grid` - A slice of rows, each row is a vector of integers.
///
/// # Returns
/// The total number of operations needed.
fn minimum_operations(grid: &[Vec<i32>]) -> i32 {
    let rows = grid.len();
    if rows == 0 || rows == 1 {
        return 0;
    }

    let cols = grid[0].len();
    let mut cal_grid: Vec<Vec<i32>> = grid.iter().map(|row| row.to_vec()).collect();
    let mut ans = 0;

    for i in 0..cols {
        for j in 1..rows {
            let prev = cal_grid[j - 1][i];
            let current = cal_grid[j][i];
            if current <= prev {
                let delta = prev + 1 - current;
                ans += delta;
                cal_grid[j][i] = prev + 1;
            }
        }
    }

    ans
}