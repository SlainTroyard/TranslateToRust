use std::io;

struct Solution;

impl Solution {
    fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let n = edges.len() + 1;
        let mut adj = vec![Vec::new(); n];
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2] as i64;
            adj[u].push((v, w));
            adj[v].push((u, w));
        }
        let mut dp = vec![[0, 0]; n];
        Self::dfs(&adj, 0, -1, k, &mut dp);
        dp[0][0]
    }

    fn dfs(
        adj: &Vec<Vec<(usize, i64)>>,
        u: usize,
        parent: i32,
        k: i32,
        dp: &mut Vec<[i64; 2]>,
    ) {
        let mut differences = Vec::new();
        let mut sum_weights = 0;
        for (v, w) in adj[u].iter() {
            if *v as i32 == parent {
                continue;
            }
            Self::dfs(adj, *v, u as i32, k, dp);
            let diff = w + dp[*v][1] - dp[*v][0];
            differences.push(diff);
            sum_weights += dp[*v][0];
        }
        differences.sort_by(|a, b| b.cmp(a)); // Sort in descending order
        let max_take = std::cmp::min(k as usize, differences.len());
        let mut take = 0;
        for i in 0..max_take {
            if differences[i] > 0 {
                take += differences[i];
            }
        }
        dp[u][0] = sum_weights + take;
        let max_take_1 = std::cmp::min((k - 1) as usize, differences.len());
        let mut take_1 = 0;
        for i in 0..max_take_1 {
            if differences[i] > 0 {
                take_1 += differences[i];
            }
        }
        dp[u][1] = sum_weights + take_1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        });
    let first_line = lines.next().unwrap();
    let n = first_line[0] as usize;
    let k = first_line[1];
    let mut edges = Vec::with_capacity(n);
    for _ in 0..n {
        let parts = lines.next().unwrap();
        let u = parts[0];
        let v = parts[1];
        let w = parts[2];
        edges.push(vec![u, v, w]);
    }
    let solution = Solution;
    let result = solution.maximize_sum_of_weights(edges, k);
    println!("{}", result);
}