use std::io;

/// Translates the given C code into idiomatic Rust, preserving the zigzag traversal algorithm.
///
/// The function reads grid dimensions and elements from stdin, processes them using zigzag traversal,
/// and prints the result to stdout as space-separated integers.
///
/// # Examples
///
/// Input:
/// 3 4
/// 0 1 2 3 4 5 6 7 8 9 10 11
///
/// Output:
/// 0 2 7 5 8 10
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut iter = numbers.iter();
    let grid_size = *iter.next().unwrap() as usize;
    let grid_col_size = *iter.next().unwrap() as usize;
    let total_elements_needed = grid_size * grid_col_size;
    assert!(
        numbers.len() - 2 == total_elements_needed,
        "Input has incorrect number of elements"
    );

    // Construct the grid from the input numbers
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let row: Vec<i32> = iter
            .by_ref()
            .take(grid_col_size)
            .cloned()
            .collect();
        grid.push(row);
    }

    // Perform zigzag traversal
    let ans = zigzag_traversal(&grid, grid_col_size);

    // Print the result with space-separated integers
    for (i, &num) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", num);
    }
    println!();
}

/// Performs a zigzag traversal on the given grid.
///
/// # Arguments
///
/// * `grid` - A reference to the 2D grid to traverse.
/// * `grid_col_size` - The number of columns in each row of the grid.
///
/// # Returns
///
/// A vector containing the elements of the grid in zigzag order.
fn zigzag_traversal(grid: &Vec<Vec<i32>>, grid_col_size: usize) -> Vec<i32> {
    let mut ans = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        if r % 2 == 0 {
            // Traverse even rows left to right, stepping by 2
            for c in (0..grid_col_size).step_by(2) {
                ans.push(row[c]);
            }
        } else {
            // Traverse odd rows right to left, stepping by 2
            let c1 = (grid_col_size - 1) - (grid_col_size & 1);
            let mut c = c1;
            while c >= 0 {
                ans.push(row[c]);
                c -= 2;
            }
        }
    }
    ans
}