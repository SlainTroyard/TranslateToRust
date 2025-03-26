use std::io::{self, Read};

fn main() {
    // Read all input into a string and split into tokens
    let input = {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).expect("Failed to read input");
        s
    };
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    // Read matrix dimensions
    let m = tokens.next().unwrap();
    let n = tokens.next().unwrap();

    // Read matrix values into a 2D vector
    let mut grid = Vec::with_capacity(m as usize);
    for _ in 0..m {
        let mut row = Vec::with_capacity(n as usize);
        for _ in 0..n {
            row.push(tokens.next().unwrap());
        }
        grid.push(row);
    }

    // Calculate and print the result
    println!("{}", minimum_operations(&grid));
}

fn minimum_operations(grid: &Vec<Vec<i32>>) -> i32 {
    // Create a mutable clone of the grid to track required changes
    let mut cal_grid = grid.clone();
    let mut ans = 0;
    let m = cal_grid.len();
    
    // Early return if there's only one row or no rows
    if m <= 1 {
        return 0;
    }
    let n = cal_grid[0].len();

    // Process each column to ensure strictly increasing values
    for col in 0..n {
        for row in 1..m {
            let current = cal_grid[row][col];
            let prev = cal_grid[row - 1][col];
            
            if current <= prev {
                // Calculate required value and update answer/grid
                let needed = prev + 1;
                ans += needed - current;
                cal_grid[row][col] = needed;
            }
        }
    }

    ans
}