use std::io;
use std::collections::BinaryHeap;

fn sort_matrix(grid: &mut Vec<Vec<i32>>, grid_size: usize) {
    // Sorting diagonals from top-left to bottom-right
    for i in 0..grid_size {
        let mut diagonal = Vec::new();
        let mut x = i;
        let mut y = 0;
        while x < grid_size && y < grid_size {
            diagonal.push(grid[x][y]);
            x += 1;
            y += 1;
        }
        // Sort descending
        diagonal.sort_by(|a, b| b.cmp(a));
        let mut index = 0;
        let mut x = i;
        let mut y = 0;
        while x < grid_size && y < grid_size {
            grid[x][y] = diagonal[index];
            index += 1;
            x += 1;
            y += 1;
        }
    }

    // Sorting diagonals from top-right to bottom-left
    for i in 1..grid_size {
        let mut diagonal = Vec::new();
        let mut x = 0;
        let mut y = i;
        while x < grid_size && y < grid_size {
            diagonal.push(grid[x][y]);
            x += 1;
            y += 1;
        }
        // Sort ascending
        diagonal.sort();
        let mut index = 0;
        let mut x = 0;
        let mut y = i;
        while x < grid_size && y < grid_size {
            grid[x][y] = diagonal[index];
            index += 1;
            x += 1;
            y += 1;
        }
    }
}

fn main() {
    // Reading input
    let mut input = String::new();
    let stdin = io::stdin();

    // Read the grid size (n)
    stdin.read_line(&mut input).expect("Failed to read line");
    let grid_size: usize = input.trim().parse().expect("Invalid grid size");

    // Initialize the grid
    let mut grid = Vec::new();

    // Read the grid values
    for _ in 0..grid_size {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        grid.push(row);
    }

    // Sort the matrix according to the algorithm
    sort_matrix(&mut grid, grid_size);

    // Print the result
    for row in grid {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}