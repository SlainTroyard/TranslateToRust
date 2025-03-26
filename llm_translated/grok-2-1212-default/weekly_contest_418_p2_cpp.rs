use std::io::{self, BufRead};

struct Solution;

impl Solution {
    fn remaining_methods(n: i32, k: i32, invocations: &Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let k = k as usize;
        let mut graph = vec![Vec::new(); n];
        for e in invocations {
            graph[e[0] as usize].push(e[1] as usize);
        }

        // Mark all suspicious methods
        let mut is_suspicious = vec![false; n];
        let mut dfs = |x: usize| {
            is_suspicious[x] = true;
            for &y in &graph[x] {
                if !is_suspicious[y] { // Avoid infinite recursion
                    dfs(y);
                }
            }
        };
        dfs(k);

        // Check if there's an edge from a non-suspicious method to a suspicious method
        for e in invocations {
            if !is_suspicious[e[0] as usize] && is_suspicious[e[1] as usize] {
                // Cannot remove suspicious methods
                return (0..n as i32).collect();
            }
        }

        // Remove all suspicious methods
        let mut ans = Vec::new();
        for i in 0..n {
            if !is_suspicious[i] {
                ans.push(i as i32);
            }
        }
        ans
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap()?;
    let mut numbers = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = numbers.next().unwrap();
    let k = numbers.next().unwrap();
    let invocations_size = numbers.next().unwrap();

    let mut invocations = Vec::new();
    for _ in 0..invocations_size {
        let line = lines.next().unwrap()?;
        let mut numbers = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();
        invocations.push(vec![a, b]);
    }

    let ans = Solution::remaining_methods(n, k, &invocations);
    for x in ans {
        print!("{} ", x);
    }
    println!();

    Ok(())
}