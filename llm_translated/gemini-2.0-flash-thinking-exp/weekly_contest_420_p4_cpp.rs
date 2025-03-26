// Problem: Weekly Contest 420 Problem 4
use std::io;
use std::io::prelude::*;
use std::vec;

struct Solution {}

impl Solution {
    pub fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
        let n = parent.len();
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 1..n {
            let p = parent[i] as usize;
            g[p].push(i);
        }

        let mut dfs_str: Vec<char> = vec!['\0'; n];
        let mut nodes: Vec<(usize, usize)> = vec![(0, 0); n];
        let mut time = 0;

        fn dfs_recursive(
            x: usize,
            g: &Vec<Vec<usize>>,
            s: &String,
            dfs_str: &mut Vec<char>,
            nodes: &mut Vec<(usize, usize)>,
            time: &mut usize,
        ) {
            nodes[x].0 = *time;
            for &y in &g[x] {
                dfs_recursive(y, g, s, dfs_str, nodes, time);
            }
            dfs_str[*time] = s.chars().nth(x).unwrap();
            *time += 1;
            nodes[x].1 = *time;
        }

        dfs_recursive(0, &g, &s, &mut dfs_str, &mut nodes, &mut time);

        let mut t = String::from("^");
        for c in &dfs_str {
            t.push('#');
            t.push(*c);
        }
        t.push_str("#$");

        let t_len = t.len();
        let mut half_len: Vec<i32> = vec![0; t_len];
        half_len[1] = 1;
        let mut box_m = 0;
        let mut box_r = 0;

        for i in 2..(t_len - 1) {
            let mut hl = 1;
            if i < box_r {
                hl = std::cmp::min(half_len[2 * box_m - i], box_r as i32 - i as i32) as i32;
            }
            while t.chars().nth((i as i32 - hl) as usize) == t.chars().nth((i as i32 + hl) as usize) {
                hl += 1;
                box_m = i;
                box_r = i + hl as usize;
                if (i as i32 + hl) >= (t_len -1) as i32 {
                    break; // Prevent out of bounds access
                }
            }
            half_len[i] = hl - 1;
        }

        let is_palindrome = |l: usize, r: usize| -> bool {
            if l > r {
                return false; // Handle invalid range
            }
            if l == r {
                return true; // Single character is always palindrome
            }
            half_len[l + r + 1] > (r - l) as i32
        };

        let mut ans: Vec<bool> = vec![false; n];
        for i in 0..n {
            ans[i] = is_palindrome(nodes[i].0, nodes[i].1);
        }
        ans
    }
}

fn main() {
    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input for n");

    let mut parent_str = String::new();
    io::stdin().read_line(&mut parent_str).expect("Failed to read line");
    let parent: Vec<i32> = parent_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for parent"))
        .collect();

    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim().to_string();

    let solution = Solution {};
    let ans = solution.find_answer(parent, s);

    for b in ans {
        if b {
            print!("true ");
        } else {
            print!("false ");
        }
    }
    println!();
}