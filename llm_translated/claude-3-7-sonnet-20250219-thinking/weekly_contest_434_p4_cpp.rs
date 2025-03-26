use std::io;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut all = 0;
        let mut mask2 = 0;
        let mut g: Vec<Vec<i32>> = vec![Vec::new(); 26];
        
        // Parse each word and build the graph
        for s in &words {
            let x = (s.chars().nth(0).unwrap() as u8 - b'a') as usize;
            let y = (s.chars().nth(1).unwrap() as u8 - b'a') as usize;
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x].push(y as i32);
        }

        // Helper function for cycle detection using DFS
        fn dfs(x: usize, sub: i32, color: &mut [i32; 26], g: &[Vec<i32>]) -> bool {
            color[x] = 1; // Mark as being visited
            for &y in &g[x] {
                let y = y as usize;
                if (sub >> y & 1) == 0 {
                    continue;
                }
                if color[y] == 1 || (color[y] == 0 && dfs(y, sub, color, g)) {
                    return true; // Cycle detected
                }
            }
            color[x] = 2; // Mark as fully visited
            false
        }
        
        // Check if a subset has a cycle
        let has_cycle = |sub: i32| -> bool {
            let mut color = [0; 26];
            
            for i in 0..26 {
                if (sub >> i & 1) != 0 && color[i] == 0 && dfs(i, sub, &mut color, &g) {
                    return true;
                }
            }
            false
        };
        
        let mut st = HashSet::new();
        let mut max_size = 0;
        let mask1 = all ^ mask2;
        let mut sub = mask1;
        
        // Iterate through all possible subsets
        loop {
            let size = sub.count_ones() as i32;
            if size >= max_size && !has_cycle(sub) {
                if size > max_size {
                    max_size = size;
                    st.clear();
                }
                st.insert(sub);
            }
            sub = (sub - 1) & mask1;
            if sub == mask1 {
                break;
            }
        }
        
        // Construct the answer
        let mut ans = Vec::new();
        for sub in st {
            let mut cnt = vec![0; 26];
            for i in 0..26 {
                cnt[i] = ((all >> i & 1) + ((all ^ sub) >> i & 1)) as i32;
            }
            ans.push(cnt);
        }
        ans
    }
}

fn main() {
    // Read number of words
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Input not an integer");
    
    // Read each word
    let mut words = Vec::new();
    for _ in 0..n {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        words.push(word.trim().to_string());
    }
    
    // Solve the problem
    let sol = Solution;
    let ans = sol.supersequences(words);
    
    // Print the result
    for cnt in ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }
}