use std::collections::HashSet;
use std::io::{self, BufRead};

/// Calculates the number of 1s in the binary representation of a number.
fn popcount(mut n: u32) -> u32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

/// Checks if a directed graph has a cycle using non-recursive DFS.
fn has_cycle(sub: u32, graph: &[Vec<usize>]) -> bool {
    let mut color = vec![0; 26]; // 0: unvisited, 1: visiting, 2: visited

    for start in 0..26 {
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = vec![start];
        let mut stack_pos = vec![0];
        color[start] = 1; // Mark as visiting

        while let Some(&x) = stack.last() {
            if stack_pos.last().unwrap() >= &graph[x].len() {
                color[x] = 2; // Mark as visited
                stack.pop();
                stack_pos.pop();
                continue;
            }

            let y = graph[x][stack_pos.last_mut().unwrap()];
            *stack_pos.last_mut().unwrap() += 1;

            if (sub >> y & 1) == 0 {
                continue;
            }

            if color[y] == 1 {
                return true; // Cycle detected
            }

            if color[y] == 0 {
                color[y] = 1; // Mark as visiting
                stack.push(y);
                stack_pos.push(0);
            }
        }
    }

    false
}

/// Main function to calculate valid subsets and their supersequences.
fn supersequences(words: Vec<String>) -> (Vec<Vec<u32>>, Vec<usize>) {
    let mut all = 0;
    let mut mask2 = 0;
    let mut graph = vec![Vec::new(); 26];

    // Build graph and calculate masks
    for word in &words {
        let x = (word.chars().nth(0).unwrap() as u8 - b'a') as usize;
        let y = (word.chars().nth(1).unwrap() as u8 - b'a') as usize;

        all |= (1 << x) | (1 << y);

        if x == y {
            mask2 |= 1 << x;
        }

        graph[x].push(y);
    }

    let mask1 = all ^ mask2;

    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    // Enumerate all subsets of mask1
    let mut sub = mask1;
    loop {
        let size = popcount(sub);

        if size >= max_size && !has_cycle(sub, &graph) {
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
    let mut column_sizes = Vec::new();

    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        for j in 0..26 {
            row[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
        }
        result.push(row);
        column_sizes.push(26);
    }

    (result, column_sizes)
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of words
    let n: usize = lines
        .next()
        .expect("Failed to read input")
        .expect("Failed to parse input")
        .trim()
        .parse()
        .expect("Failed to parse number of words");

    // Read the words
    let mut words = Vec::new();
    for _ in 0..n {
        let word = lines
            .next()
            .expect("Failed to read input")
            .expect("Failed to parse input")
            .trim()
            .to_string();
        words.push(word);
    }

    // Calculate results
    let (result, column_sizes) = supersequences(words);

    // Print results
    for (row, &size) in result.iter().zip(&column_sizes) {
        for j in 0..size {
            print!("{} ", row[j]);
        }
        println!();
    }
}