use std::collections::HashMap;
use std::io;

struct DP {
    all_nums: Vec<i32>,
    pos: HashMap<i32, i32>,
    memo: Vec<Vec<i32>>,
}

impl DP {
    fn new(all_nums: Vec<i32>, pos: HashMap<i32, i32>, m: usize) -> Self {
        let n = all_nums.len();
        let max_mask = 1 << m;
        let memo = vec![vec![-1; max_mask]; n];
        DP {
            all_nums,
            pos,
            memo,
        }
    }

    fn dfs(&mut self, i: usize, j: i32) -> i32 {
        if i == 0 {
            let x = self.all_nums[i];
            let t = self.pos[&x];
            let mut current_t = t;
            let mut max_score = 0;
            while current_t != 0 {
                let lb = current_t & -current_t;
                if (j & lb) == 0 {
                    max_score = max_score.max(x);
                }
                current_t ^= lb;
            }
            return max_score;
        }

        if self.memo[i][j as usize] != -1 {
            return self.memo[i][j as usize];
        }

        let mut res = self.dfs(i - 1, j);

        let x = self.all_nums[i];
        let t = self.pos[&x];
        let mut current_t = t;
        while current_t != 0 {
            let lb = current_t & -current_t;
            if (j & lb) == 0 {
                let new_j = j | lb;
                res = res.max(self.dfs(i - 1, new_j) + x);
            }
            current_t ^= lb;
        }

        self.memo[i][j as usize] = res;
        res
    }
}

fn main() {
    let m = io::stdin()
        .read_line()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let mut grid = Vec::with_capacity(m);
    for _ in 0..m {
        let line = io::stdin()
            .read_line()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let col_size = line[0] as usize;
        let row = line[1..col_size + 1].to_vec();
        grid.push(row);
    }

    let mut pos = HashMap::new();
    for i in 0..m {
        for x in &grid[i] {
            let entry = pos.entry(*x).or_insert(0);
            *entry |= 1 << i;
        }
    }

    let all_nums: Vec<i32> = pos.keys().cloned().collect();
    let n = all_nums.len();
    let mut dp = DP::new(all_nums, pos, m);
    let result = dp.dfs(n - 1, 0);
    println!("{}", result);
}