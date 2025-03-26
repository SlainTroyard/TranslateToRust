// Problem: Weekly Contest 426 Problem 3 in Rust

use std::io;
use std::io::prelude::*;
use std::vec::Vec;

fn linepots(k: i32, pots: &Vec<Vec<i32>>, node: i32, visited: i32) -> i32 {
    if k == -1 {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    let mut answer = 1;
    if node >= 0 && node < pots.len() as i32 {
        for &neighbor in &pots[node as usize] {
            if neighbor != visited {
                answer += linepots(k - 1, pots, neighbor, node);
            }
        }
    }
    answer
}

fn max_target_nodes(
    edges1: &Vec<Vec<i32>>,
    edges2: &Vec<Vec<i32>>,
    k: i32,
) -> Vec<i32> {
    let mut len1 = 0;
    let mut len2 = 0;

    for edge in edges1 {
        if edge[1] > len1 {
            len1 = edge[1];
        }
    }
    for edge in edges2 {
        if edge[1] > len2 {
            len2 = edge[1];
        }
    }

    let mut pots1: Vec<Vec<i32>> = vec![Vec::new(); (len1 + 1) as usize];
    let mut pots2: Vec<Vec<i32>> = vec![Vec::new(); (len2 + 1) as usize];

    for i in 0..=len1 {
        let mut ccc: Vec<i32> = Vec::new();
        for edge in edges1 {
            if edge[0] == i {
                ccc.push(edge[1]);
            }
            if edge[1] == i {
                ccc.push(edge[0]);
            }
        }
        pots1[i as usize] = ccc;
    }

    for i in 0..=len2 {
        let mut ccc: Vec<i32> = Vec::new();
        for edge in edges2 {
            if edge[0] == i {
                ccc.push(edge[1]);
            }
            if edge[1] == i {
                ccc.push(edge[0]);
            }
        }
        pots2[i as usize] = ccc;
    }

    let mut max_val = 0;
    for i in 0..=len2 {
        let temp = linepots(k - 1, &pots2, i, -1);
        if temp > max_val {
            max_val = temp;
        }
    }

    let mut answer: Vec<i32> = vec![0; (len1 + 1) as usize];
    for i in 0..=len1 {
        answer[i as usize] = linepots(k, &pots1, i, -1) + max_val;
    }

    answer
}

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut n1_str = String::new();
    stdin.read_line(&mut n1_str).unwrap();
    let n1: i32 = n1_str.trim().parse().unwrap();

    let mut edges1: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n1 {
        let mut edge_str = String::new();
        stdin.read_line(&mut edge_str).unwrap();
        let edge_vec: Vec<i32> = edge_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges1.push(edge_vec);
    }

    let mut n2_str = String::new();
    stdin.read_line(&mut n2_str).unwrap();
    let n2: i32 = n2_str.trim().parse().unwrap();

    let mut edges2: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n2 {
        let mut edge_str = String::new();
        stdin.read_line(&mut edge_str).unwrap();
        let edge_vec: Vec<i32> = edge_str
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        edges2.push(edge_vec);
    }

    let mut k_str = String::new();
    stdin.read_line(&mut k_str).unwrap();
    let k: i32 = k_str.trim().parse().unwrap();

    let result = max_target_nodes(&edges1, &edges2, k);

    for i in 0..result.len() {
        write!(stdout, "{} ", result[i]).unwrap();
    }
    writeln!(stdout).unwrap();
}