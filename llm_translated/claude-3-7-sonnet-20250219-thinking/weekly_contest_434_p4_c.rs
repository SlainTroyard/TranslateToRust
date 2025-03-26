use std::io::{self, BufRead};

// Count the number of 1s in the binary representation
fn popcount(n: i32) -> i32 {
    n.count_ones() as i32
}

// Check if a directed graph has a cycle
fn has_cycle(sub: i32, g: &[[i32; 26]; 26], g_size: &[i32; 26]) -> bool {
    let mut color = [0; 26]; // 0: unvisited, 1: visiting, 2: visited
    
    // Use non-recursive DFS to detect cycles
    for start in 0..26 {
        // Skip if node is not in subset or already completely visited
        if (sub >> start & 1) == 0 || color[start] == 2 {
            continue;
        }
        
        // Use stack instead of recursion
        let mut stack = [0; 26];
        let mut stack_pos = [0; 26]; // Current position in neighbor list for each node
        let mut top = 0;
        
        stack[top] = start as i32;
        stack_pos[top] = 0;
        color[start as usize] = 1; // Mark as visiting
        
        while top >= 0 {
            let x = stack[top] as usize;
            
            // If all neighbors have been processed
            if stack_pos[top] >= g_size[x] {
                color[x] = 2; // Mark as visited
                top -= 1;
                continue;
            }
            
            let idx = stack_pos[top] as usize;
            stack_pos[top] += 1;
            let y = g[x][idx] as usize;
            
            // Skip if y is not in the current subset
            if (sub >> y & 1) == 0 {
                continue;
            }
            
            // If y is being visited, there is a cycle
            if color[y] == 1 {
                return true;
            }
            
            // If y has not been visited, push to stack
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

// Main algorithm to compute supersequences
fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = [[0; 26]; 26]; // Adjacency list
    let mut g_size = [0; 26]; // Number of neighbors for each node
    
    // Build graph and calculate masks
    for word in words {
        let chars: Vec<char> = word.chars().collect();
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
    
    // Store valid subsets
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
    
    // Build result
    let mut result = Vec::with_capacity(valid_subsets.len());
    
    for &sub in &valid_subsets {
        let mut subset_result = vec![0; 26];
        
        // Calculate count for each letter
        for j in 0..26 {
            subset_result[j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
        }
        
        result.push(subset_result);
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read n
    let n: usize = lines.next().unwrap()?.trim().parse()
        .expect("Error reading input for n");
    
    // Read words
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        let word = lines.next().unwrap()?;
        if word.len() < 2 {
            eprintln!("Error: word too short");
            return Ok(());
        }
        words.push(word);
    }
    
    // Calculate result
    let result = supersequences(&words);
    
    // Output result
    for row in result {
        for (i, val) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", val);
        }
        println!();
    }
    
    Ok(())
}