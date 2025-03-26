use std::io::{self, Read};

// Calculate the number of set bits in a u32 (not needed as we use count_ones)
// But kept to mirror original code's logic, though in practice we use count_ones()
fn popcount(n: u32) -> u32 {
    n.count_ones()
}

// Check if the subset has a cycle using non-recursive DFS
fn has_cycle(sub: u32, graph: &[Vec<usize>]) -> bool {
    let mut color = [0; 26]; // 0: unvisited, 1: visiting, 2: visited

    for start in 0..26 {
        if (sub & (1 << start)) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = Vec::new();
        let mut stack_pos = Vec::new();
        stack.push(start);
        stack_pos.push(0);
        color[start] = 1;

        while let Some(top) = stack.last().copied() {
            let pos = stack_pos.last_mut().unwrap();

            if *pos >= graph[top].len() {
                color[top] = 2;
                stack.pop();
                stack_pos.pop();
                continue;
            }

            let y = graph[top][*pos];
            *pos += 1;

            if (sub & (1 << y)) == 0 {
                continue;
            }

            match color[y] {
                1 => return true,
                0 => {
                    color[y] = 1;
                    stack.push(y);
                    stack_pos.push(0);
                }
                _ => {}
            }
        }
    }

    false
}

fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0u32;
    let mut mask2 = 0u32;
    let mut graph = vec![vec![]; 26];

    for word in words {
        let x = (word.as_bytes()[0] - b'a') as usize;
        let y = (word.as_bytes()[1] - b'a') as usize;
        all |= (1 << x) | (1 << y);
        if x == y {
            mask2 |= 1 << x;
        }
        graph[x].push(y);
    }

    let mask1 = all ^ mask2;
    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    let mut sub = mask1;
    loop {
        let size = popcount(sub) as usize;

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

        let next_sub = (sub - 1) & mask1;
        if next_sub == mask1 {
            break;
        }
        sub = next_sub;
    }

    valid_subsets
        .into_iter()
        .map(|sub| {
            (0..26)
                .map(|j| {
                    let in_all = ((all >> j) & 1) as i32;
                    let in_all_xor_sub = (((all ^ sub) >> j) & 1) as i32;
                    in_all + in_all_xor_sub
                })
                .collect()
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    let n: usize = tokens
        .next()
        .expect("No n provided")
        .parse()
        .expect("Invalid n");

    let words: Vec<String> = tokens.take(n).map(|s| s.to_string()).collect();

    if words.len() != n {
        panic!("Not enough words provided");
    }

    for word in &words {
        if word.len() != 2 {
            panic!("Word '{}' is not length 2", word);
        }
    }

    let result = supersequences(&words);

    for row in result {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
}