use std::collections::HashSet;
use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut all = 0;
        let mut mask2 = 0;
        let mut g: Vec<Vec<i32>> = vec![vec![]; 26];
        
        for s in &words {
            let chars: Vec<char> = s.chars().collect();
            let x = (chars[0] as usize) - ('a' as usize);
            let y = (chars[1] as usize) - ('a' as usize);
            
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x].push(y as i32);
        }

        // Define cycle detection function
        let has_cycle = |sub: i32| -> bool {
            let mut color = [0; 26];
            
            // Define DFS function
            fn dfs(x: usize, sub: i32, color: &mut [i32; 26], g: &Vec<Vec<i32>>) -> bool {
                color[x] = 1;
                for &y in &g[x] {
                    let y = y as usize;
                    if (sub >> y & 1) == 0 {
                        continue;
                    }
                    if color[y] == 1 || (color[y] == 0 && dfs(y, sub, color, g)) {
                        return true;
                    }
                }
                color[x] = 2;
                false
            }
            
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
        
        // Loop through all subsets of mask1
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of words
    let n: usize = lines.next().unwrap()?.parse().unwrap();
    
    // Read the words
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(lines.next().unwrap()?);
    }
    
    let ans = Solution::supersequences(words);
    
    // Print the answer
    for cnt in ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }
    
    Ok(())
}