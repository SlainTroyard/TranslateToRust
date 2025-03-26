use std::io::{self, BufRead};

/// Counts the number of 1s in the binary representation of a number
fn popcount(mut n: i32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += n & 1;
        n >>= 1;
    }
    count
}

/// Checks if the directed graph has a cycle
fn has_cycle(sub: i32, g: &[[i32; 26]; 26], g_size: &[i32; 26]) -> bool {
    let mut color = [0i32; 26]; // 0: unvisited, 1: visiting, 2: visited
    
    // Using non-recursive DFS to detect cycles
    for start in 0..26 {
        // Skip if node not in subset or already fully visited
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }
        
        // Use stack instead of recursion
        let mut stack = [0i32; 26];
        let mut stack_pos = [0i32; 26]; // Track current position in neighbor list for each node
        let mut top = 0;
        
        stack[top] = start as i32;
        stack_pos[top] = 0;
        color[start] = 1; // Mark as visiting
        
        while top >= 0 {
            let x = stack[top] as usize;
            
            // If all neighbors processed
            if stack_pos[top] >= g_size[x] {
                color[x] = 2; // Mark as visited
                top -= 1;
                continue;
            }
            
            let pos = stack_pos[top] as usize;
            stack_pos[top] += 1;
            let y = g[x][pos] as usize;
            
            // Skip if y not in current subset
            if (sub >> y & 1) == 0 {
                continue;
            }
            
            // If y is currently being visited, there's a cycle
            if color[y] == 1 {
                return true;
            }
            
            // If y is unvisited, push to stack
            if color[y] == 0 {
                color[y] = 1; // Mark as visiting
                top += 1;
                stack[top] = y as i32;
                stack_pos[top] = 0;
            }
        }
    }
    
    false
}

fn supersequences(words: &[String], words_size: usize) -> Vec<Vec<i32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = [[0i32; 26]; 26]; // Adjacency list
    let mut g_size = [0i32; 26]; // Number of neighbors for each node
    
    // Build graph and calculate masks
    for i in 0..words_size {
        let chars: Vec<char> = words[i].chars().collect();
        let x = (chars[0] as u8 - b'a') as usize;
        let y = (chars[1] as u8 - b'a') as usize;
        
        all |= (1 << x) | (1 << y);
        
        if x == y {
            mask2 |= (1 << x);
        }
        
        g[x][g_size[x] as usize] = y as i32;
        g_size[x] += 1;
    }
    
    // Calculate mask1
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
        
        // Calculate next subset of mask1
        if sub == 0 {
            break;
        }
        sub = (sub - 1) & mask1;
        if sub == mask1 {
            break;
        }
    }
    
    let mut result = Vec::new();
    
    if !valid_subsets.is_empty() {
        // Build result
        for sub in valid_subsets {
            let mut row = vec![0; 26];
            
            // Calculate occurrence count for each letter
            for j in 0..26 {
                row[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
            }
            
            result.push(row);
        }
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next()
        .expect("Failed to read input line")?
        .trim()
        .parse()
        .expect("Failed to parse n");
    
    // Read words
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let word = lines.next()
            .expect("Failed to read word")?;
        words.push(word);
    }
    
    // Call function to calculate result
    let result = supersequences(&words, n);
    
    // Output result
    for row in &result {
        for (i, &val) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", val);
        }
        println!();
    }
    
    Ok(())
}