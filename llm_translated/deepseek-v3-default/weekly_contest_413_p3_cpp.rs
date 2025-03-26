use std::collections::HashMap;
use std::io;

fn main() {
    // Read the grid size
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let grid_size: usize = input.trim().parse().expect("Invalid grid size");

    // Read the grid
    let mut grid: Vec<Vec<i32>> = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let grid_col_size: usize = input.trim().parse().expect("Invalid grid column size");
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let row: Vec<i32> = input
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number in grid"))
            .collect();
        
        if row.len() != grid_col_size {
            panic!("Grid column size mismatch");
        }
        
        grid.push(row);
    }

    // Calculate and print the max score
    let sol = Solution;
    println!("{}", sol.max_score(grid));
}

struct Solution;

impl Solution {
    pub fn max_score(&self, grid: Vec<Vec<i32>>) -> i32 {
        let mut pos: HashMap<i32, i32> = HashMap::new();
        let m = grid.len();

        // Populate the pos map
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        // Collect all unique numbers
        let all_nums: Vec<i32> = pos.keys().cloned().collect();
        let n = all_nums.len();

        // Initialize memoization table
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; 1 << m]; n];

        // Define the DFS function
        fn dfs(
            memo: &mut Vec<Vec<i32>>,
            all_nums: &Vec<i32>,
            pos: &HashMap<i32, i32>,
            i: i32,
            j: i32,
        ) -> i32 {
            if i < 0 {
                return 0;
            }
            let ui = i as usize;
            let uj = j as usize;
            if memo[ui][uj] != -1 {
                return memo[ui][uj];
            }
            let mut res = dfs(memo, all_nums, pos, i - 1, j);
            let x = all_nums[ui];
            let mut t = *pos.get(&x).unwrap();
            while t != 0 {
                let lb = t & -t;
                if (j & lb) == 0 {
                    res = res.max(dfs(memo, all_nums, pos, i - 1, j | lb) + x);
                }
                t ^= lb;
            }
            memo[ui][uj] = res;
            res
        }

        // Call the DFS function with initial parameters
        dfs(&mut memo, &all_nums, &pos, (n - 1) as i32, 0)
    }
}