use std::collections::BTreeMap;

struct Solution;

impl Solution {
    fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let mut pos = BTreeMap::new(); // Maps number to bitmask of rows it appears in
        for (i, row) in grid.iter().enumerate() {
            for &x in row {
                *pos.entry(x).or_insert(0) |= 1 << i;
            }
        }

        let all_nums: Vec<i32> = pos.keys().cloned().collect(); // Sorted if using BTreeMap
        let n = all_nums.len();
        let max_mask = 1 << m;
        let mut memo = vec![vec![-1; max_mask]; n];

        let dfs = |dfs: &dyn Fn(usize, usize) -> i32, i: usize, j: usize| -> i32 {
            if i == 0 {
                return 0;
            }
            let res = &mut memo[i - 1][j];
            if *res != -1 {
                return *res;
            }
            let mut res_val = dfs(dfs, i - 1, j);
            let x = all_nums[i - 1];
            let t = *pos.get(&x).unwrap();
            let mut current_t = t;
            while current_t != 0 {
                let lb = current_t & -current_t;
                if (j & (lb as usize)) == 0 {
                    let new_j = j | (lb as usize);
                    let candidate = dfs(dfs, i - 1, new_j) + x;
                    if candidate > res_val {
                        res_val = candidate;
                    }
                }
                current_t ^= lb;
            }
            *res = res_val;
            res_val
        };

        // Edge case: when there are no numbers (n=0)
        if n == 0 {
            return 0;
        }

        dfs(&dfs, n, 0)
    }
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let m: usize = lines.next().unwrap().parse().unwrap();
    let mut grid = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let col_size: usize = parts[0].parse().unwrap();
        let row: Vec<i32> = parts[1..=col_size]
            .iter()
            .map(|&s| s.parse().unwrap())
            .collect();
        grid.push(row);
    }
    let solution = Solution;
    println!("{}", solution.max_score(grid));
}