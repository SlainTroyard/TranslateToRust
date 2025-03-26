use std::cmp;

struct Solution;

impl Solution {
    fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
        let m = grid.len();
        let n = grid[0].len();
        // memo[i][j][k][can_turn] stores the max length starting at (i,j) with direction k
        let mut memo = vec![vec![vec![vec![0; 2]; 4]; n]; m];

        fn dfs(
            i: i32,
            j: i32,
            dir: usize,
            can_turn: bool,
            target: i32,
            grid: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
            rows: i32,
            cols: i32,
        ) -> i32 {
            // Check if current position is out of bounds
            if i < 0 || i >= rows || j < 0 || j >= cols {
                return 0;
            }

            // Calculate next position based on direction
            let (di, dj) = DIRS[dir];
            let ni = i + di;
            let nj = j + dj;

            // Check if next position is valid and matches target
            if ni < 0 || nj < 0 || ni >= rows || nj >= cols || grid[ni as usize][nj as usize] != target {
                return 0;
            }

            // Access memoization entry for next position
            let res = &mut memo[ni as usize][nj as usize][dir][can_turn as usize];
            if *res != 0 {
                return *res;
            }

            // Continue in current direction without turning
            *res = dfs(ni, nj, dir, can_turn, 2 - target, grid, memo, rows, cols);

            // If allowed, try turning direction clockwise once
            if can_turn {
                let new_dir = (dir + 1) % 4;
                let turn_res = dfs(ni, nj, new_dir, false, 2 - target, grid, memo, rows, cols);
                *res = cmp::max(*res, turn_res);
            }

            // Include current cell in the path length
            *res += 1;
            *res
        }

        let rows = m as i32;
        let cols = n as i32;
        let mut max_length = 0;

        // Iterate through all cells to find maximum V-diagonal length
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    // Try all four possible starting directions
                    for dir in 0..4 {
                        let current = dfs(
                            i as i32,
                            j as i32,
                            dir,
                            true,
                            2, // Next target value is 2 (since current is 1, 2-1=1? Verify problem statement)
                            &grid,
                            &mut memo,
                            rows,
                            cols,
                        ) + 1; // Add current cell to the path
                        max_length = cmp::max(max_length, current);
                    }
                }
            }
        }

        max_length
    }
}

fn main() {
    // Read input dimensions
    let (rows, cols) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap());
        (parts.next().unwrap(), parts.next().unwrap())
    };

    // Read grid data
    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let row: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(row.len(), cols);
        grid.push(row);
    }

    // Calculate and print result
    println!("{}", Solution::len_of_v_diagonal(grid));
}