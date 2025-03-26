use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of words
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read the words
    let mut words = Vec::with_capacity(n);
    for _ in 0..n {
        words.push(lines.next().unwrap().unwrap());
    }
    
    // Compute the supersequences
    let ans = supersequences(&words);
    
    // Print the results
    for cnt in ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }
}

fn supersequences(words: &[String]) -> Vec<Vec<i32>> {
    let mut all = 0;
    let mut mask2 = 0;
    let mut g = vec![Vec::new(); 26];
    
    for s in words {
        let x = s.chars().nth(0).unwrap() as usize - 'a' as usize;
        let y = s.chars().nth(1).unwrap() as usize - 'a' as usize;
        all |= 1 << x | 1 << y;
        if x == y {
            mask2 |= 1 << x;
        }
        g[x].push(y);
    }
    
    let has_cycle = |sub: i32| -> bool {
        let mut color = vec![0; 26];
        let mut stack = VecDeque::new();
        
        for i in 0..26 {
            if (sub >> i & 1) == 1 && color[i] == 0 {
                stack.push_back(i);
                while let Some(x) = stack.pop_back() {
                    if color[x] == 1 {
                        color[x] = 2;
                    } else if color[x] == 0 {
                        color[x] = 1;
                        stack.push_back(x);
                        for &y in &g[x] {
                            if (sub >> y & 1) == 1 {
                                if color[y] == 1 {
                                    return true;
                                } else if color[y] == 0 {
                                    stack.push_back(y);
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    };
    
    let mut st = HashSet::new();
    let mut max_size = 0;
    let mask1 = all ^ mask2;
    let mut sub = mask1;
    
    loop {
        let size = sub.count_ones() as usize;
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