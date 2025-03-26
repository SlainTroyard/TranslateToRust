use std::collections::HashMap;
use std::io::{self, Read};

struct Dfs<'a> {
    all_nums: &'a [i32],
    pos: &'a HashMap<i32, i32>,
    memo: &'a mut Vec<Vec<i32>>,
}

impl<'a> Dfs<'a> {
    fn dfs(&mut self, i: i32, j: i32) -> i32 {
        if i < 0 {
            return 0;
        }
        let i_usize = i as usize;
        let j_usize = j as usize;
        if self.memo[i_usize][j_usize] != -1 {
            return self.memo[i_usize][j_usize];
        }
        let mut res = self.dfs(i - 1, j);
        let x = self.all_nums[i_usize];
        let t = *self.pos.get(&x).unwrap_or(&0);
        let mut remaining = t;
        while remaining != 0 {
            let lb = remaining & -remaining;
            if (j & lb) == 0 {
                res = res.max(self.dfs(i - 1, j | lb) + x);
            }
            remaining ^= lb;
        }
        self.memo[i_usize][j_usize] = res;
        res
    }
}

fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let mut pos = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for &x in row {
            *pos.entry(x).or_insert(0) |= 1 << i;
        }
    }

    let mut all_nums: Vec<_> = pos.keys().cloned().collect();
    let n = all_nums.len();
    if n == 0 {
        return 0;
    }

    let mut memo = vec![vec![-1; 1 << m]; n];
    let mut dfs = Dfs {
        all_nums: &all_nums,
        pos: &pos,
        memo: &mut memo,
    };

    dfs.dfs(n as i32 - 1, 0)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let grid_size = tokens.next().unwrap().parse::<usize>().unwrap();
    let mut grid = Vec::with_capacity(grid_size);
    for _ in 0..grid_size {
        let col_size = tokens.next().unwrap().parse::<usize>().unwrap();
        let mut row = Vec::with_capacity(col_size);
        for _ in 0..col_size {
            row.push(tokens.next().unwrap().parse::<i32>().unwrap());
        }
        grid.push(row);
    }
    println!("{}", max_score(grid));
}