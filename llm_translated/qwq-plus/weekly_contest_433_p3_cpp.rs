use std::io;

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, cost: Vec<Vec<i32>>) -> i64 {
        let m = (n as usize) / 2;
        let mut memo = vec![vec![vec![-1i64; 4]; 4]; m];

        let n_i32 = n;

        fn dfs(
            i: i32,
            pre_j: usize,
            pre_k: usize,
            memo: &mut Vec<Vec<Vec<i64>>>,
            cost: &Vec<Vec<i32>>,
            n: i32,
        ) -> i64 {
            if i < 0 {
                return 0;
            }
            let i_usize = i as usize;
            let current = &mut memo[i_usize][pre_j][pre_k];
            if *current != -1 {
                return *current;
            }

            let mut min_res = i64::MAX;
            for j in 0..3 {
                if j == pre_j {
                    continue;
                }
                for k in 0..3 {
                    if k != pre_k && k != j {
                        let next_i = i - 1;
                        let temp = dfs(
                            next_i,
                            j,
                            k,
                            memo,
                            cost,
                            n,
                        ) + cost[i_usize][j] as i64 + cost[(n as usize - 1 - i_usize)][k] as i64;
                        if temp < min_res {
                            min_res = temp;
                        }
                    }
                }
            }

            *current = min_res;
            min_res
        }

        dfs((n / 2 - 1) as i32, 3, 3, &mut memo, &cost, n_i32)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let n: i32 = lines.next().unwrap().parse().unwrap();
    let mut cost = Vec::new();
    for _ in 0..n {
        let line = lines.next().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        cost.push(nums);
    }

    let solution = Solution;
    let result = solution.min_cost(n, cost);
    println!("{}", result);
}