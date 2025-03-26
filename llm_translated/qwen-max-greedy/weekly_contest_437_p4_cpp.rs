use std::io::{self, BufRead, Write};

fn len_of_v_diagonal(grid: &Vec<Vec<i32>>) -> i32 {
    const DIRS: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
    let m = grid.len();
    let n = grid[0].len();

    let mut memo: Vec<Vec<Vec<[i32; 2]>>> = vec![vec![vec![[0; 2]; 4]; n]; m];

    fn dfs(
        i: isize,
        j: isize,
        k: usize,
        can_turn: bool,
        target: i32,
        grid: &Vec<Vec<i32>>,
        memo: &mut Vec<Vec<Vec<[i32; 2]>>>,
        dirs: &[(isize, isize); 4],
    ) -> i32 {
        if i < 0 || i >= grid.len() as isize || j < 0 || j >= grid[0].len() as isize {
            return 0;
        }

        let ni = i + dirs[k].0;
        let nj = j + dirs[k].1;

        if ni < 0 || nj < 0 || ni >= grid.len() as isize || nj >= grid[0].len() as isize || grid[ni as usize][nj as usize] != target {
            return 0;
        }

        let res = &mut memo[ni as usize][nj as usize][k];
        if res[can_turn as usize] != 0 {
            return res[can_turn as usize];
        }

        res[can_turn as usize] = dfs(ni, nj, k, can_turn, 2 - target, grid, memo, dirs);

        if can_turn {
            let nk = (k + 1) % 4;
            res[can_turn as usize] = res[can_turn as usize].max(dfs(ni, nj, nk, false, 2 - target, grid, memo, dirs));
        }

        res[can_turn as usize] += 1;
        res[can_turn as usize]
    }

    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                for k in 0..4 {
                    ans = ans.max(dfs(i as isize, j as isize, k, true, 2, grid, &mut memo, &DIRS) + 1);
                }
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = stdin.lock().lines();

    // Read the dimensions of the grid
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        for (j, val) in line.split_whitespace().enumerate() {
            grid[i][j] = val.parse().unwrap();
        }
    }

    // Solve the problem
    let sol = len_of_v_diagonal(&grid);

    // Output the result
    writeln!(stdout, "{}", sol).unwrap();
}