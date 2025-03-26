use std::collections::VecDeque;
use std::io::{self, Write};

// Function to count the number of 1s in the binary representation of a number
fn popcount(n: u32) -> u32 {
    n.count_ones()
}

// Function to check if a directed graph has a cycle using DFS
fn has_cycle(sub: u32, graph: &[Vec<u32>], g_size: &[u32]) -> bool {
    let mut color = vec![0; 26]; // 0: unvisited, 1: visiting, 2: visited

    for start in 0..26 {
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = VecDeque::new();
        let mut stack_pos = VecDeque::new();

        stack.push_back(start);
        stack_pos.push_back(0);
        color[start] = 1;

        while let Some(x) = stack.pop_back() {
            let pos = stack_pos.pop_back().unwrap();

            if pos >= g_size[x] as usize {
                color[x] = 2;
                continue;
            }

            let y = graph[x][pos] as usize;
            stack_pos.push_back(pos + 1);

            if (sub >> y & 1) == 0 {
                continue;
            }

            if color[y] == 1 {
                return true;
            }

            if color[y] == 0 {
                color[y] = 1;
                stack.push_back(y);
                stack_pos.push_back(0);
            }
        }
    }

    false
}

// Main function to compute supersequences
fn supersequences(words: &[String]) -> Vec<Vec<u32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut graph = vec![vec![]; 26];
    let mut g_size = vec![0; 26];

    for word in words {
        let x = (word.chars().nth(0).unwrap() as u8 - b'a') as u32;
        let y = (word.chars().nth(1).unwrap() as u8 - b'a') as u32;

        all |= (1 << x) | (1 << y);

        if x == y {
            mask2 |= (1 << x);
        }

        graph[x as usize].push(y);
        g_size[x as usize] += 1;
    }

    let mask1 = all ^ mask2;
    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    let mut sub = mask1;
    loop {
        let size = popcount(sub);

        if size >= max_size && !has_cycle(sub, &graph, &g_size) {
            if size > max_size {
                max_size = size;
                valid_subsets.clear();
            }
            valid_subsets.push(sub);
        }

        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
    }

    let mut result = Vec::new();

    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        for j in 0..26 {
            row[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
        }
        result.push(row);
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input for n");

    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read input");
        words.push(word.trim().to_string());
    }

    let result = supersequences(&words);

    for row in result {
        for val in row {
            print!("{} ", val);
        }
        println!();
    }
}