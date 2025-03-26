use std::collections::HashMap;
use std::io::{self, BufRead};

fn max_score(grid: &Vec<Vec<i32>>) -> i32 {
    let mut pos = HashMap::new();
    let m = grid.len();
    for (i, row) in grid.iter().enumerate() {
        for &x in row {
            *pos.entry(x).or_insert(0) |= 1 << i;
        }
    }

    let all_nums: Vec<i32> = pos.keys().cloned().collect();
    let n = all_nums.len();
    let mut memo = vec![vec![-1; 1 << m]; n];

    let dfs = |dfs: &dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&mut dyn Fn(), usize, usize) -> i32, usize, usize) -> i32, usize, usize) -> i32, usize, usize) -> i32, i: usize, j: usize| -> i32 {
        if i < 1 {
            return 0;
        }
        let res = &mut memo[i][j];
        if *res != -1 {
            return *res;
        }
        *res = dfs(dfs, i - 1, j);
        let x = all_nums[i];
        let t = pos[&x];
        let mut lb = 1;
        while t > 0 {
            lb = t & -t;
            if (j & lb) == 0 {
                *res = (*res).max(dfs(dfs, i - 1, j | lb) + x);
            }
            t ^= lb;
        }
        *res
    };

    dfs(&dfs, n - 1, 0)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let gridSize: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut grid = Vec::with_capacity(gridSize);

    for _ in 0..gridSize {
        let gridColSize: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut row = Vec::with_capacity(gridColSize);
        for _ in 0..gridColSize {
            let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
            row.push(value);
        }
        grid.push(row);
    }

    let sol = max_score(&grid);
    println!("{}", sol);
}