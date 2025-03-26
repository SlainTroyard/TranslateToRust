use std::io::{self, BufRead};

// Function to count the number of set bits in an integer
fn popcount(n: u32) -> u32 {
    let mut count = 0;
    let mut n = n;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

// Function to check if a directed graph has a cycle
fn has_cycle(sub: u32, g: &[[usize; 26]; 26], g_size: &[usize; 26]) -> bool {
    let mut color = [0; 26]; // 0: unvisited, 1: visiting, 2: visited

    for start in 0..26 {
        if (sub & (1 << start)) == 0 || color[start] == 2 {
            continue;
        }

        let mut stack = vec![start];
        let mut stack_pos = vec![0];
        color[start] = 1; // Mark as visiting

        while let Some(&x) = stack.last() {
            if stack_pos.last().unwrap() >= &g_size[x] {
                color[x] = 2; // Mark as visited
                stack.pop();
                stack_pos.pop();
                continue;
            }

            let y = g[x][*stack_pos.last().unwrap()];
            *stack_pos.last_mut().unwrap() += 1;

            if (sub & (1 << y)) == 0 {
                continue;
            }

            if color[y] == 1 {
                return true;
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

// Function to find supersequences
fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0u32;
    let mut mask2 = 0u32;
    let mut g = [[0; 26]; 26];
    let mut g_size = [0; 26];

    // Build the graph and calculate masks
    for word in words {
        let x = word.chars().nth(0).unwrap() as usize - 'a' as usize;
        let y = word.chars().nth(1).unwrap() as usize - 'a' as usize;

        all |= 1 << x;
        all |= 1 << y;

        if x == y {
            mask2 |= 1 << x;
        }

        g[x][g_size[x]] = y;
        g_size[x] += 1;
    }

    let mask1 = all ^ mask2;

    let mut valid_subsets = Vec::new();
    let mut max_size = 0;

    // Enumerate all subsets of mask1
    let mut sub = mask1;
    loop {
        let size = popcount(sub);

        if size >= max_size && !has_cycle(sub, &g, &g_size) {
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

    // Build the result
    let mut result = Vec::new();
    for &sub in &valid_subsets {
        let mut row = vec![0; 26];
        for j in 0..26 {
            row[j] = ((all >> j) & 1) as i32 + (((all ^ sub) >> j) & 1) as i32;
        }
        result.push(row);
    }

    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read input
    let n: usize = lines.next().unwrap()?.parse().unwrap();

    // Read words
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(lines.next().unwrap()?.trim().to_string());
    }

    // Calculate result
    let result = supersequences(&words);

    // Output result
    for row in result {
        for &val in &row {
            print!("{} ", val);
        }
        println!();
    }

    Ok(())
}