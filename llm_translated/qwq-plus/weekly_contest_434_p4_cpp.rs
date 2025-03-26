use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut all: u32 = 0;
        let mut mask2: u32 = 0;
        let mut g: Vec<Vec<usize>> = vec![vec![]; 26];

        for word in &words {
            let x = (word.chars().nth(0).unwrap() as u8 - b'a') as usize;
            let y = (word.chars().nth(1).unwrap() as u8 - b'a') as usize;
            all |= 1 << x | 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x].push(y);
        }

        let mask1 = all ^ mask2;

        let has_cycle = |sub: u32| -> bool {
            let mut color = [0u8; 26];
            for i in 0..26 {
                if (sub & (1 << i)) == 0 {
                    continue;
                }
                if color[i] == 0 {
                    let mut stack = vec![(i, false)];
                    while let Some((v, is_processed)) = stack.pop() {
                        if is_processed {
                            color[v] = 2;
                            continue;
                        }
                        if color[v] == 1 {
                            return true;
                        }
                        color[v] = 1;
                        stack.push((v, true));
                        for &y in g[v].iter().rev() {
                            if (sub & (1 << y)) == 0 {
                                continue;
                            }
                            if color[y] == 1 {
                                return true;
                            }
                            if color[y] == 0 {
                                stack.push((y, false));
                            }
                        }
                    }
                }
            }
            false
        };

        let mut st = HashSet::new();
        let mut max_size = 0;
        let mut sub = mask1;
        loop {
            let size = sub.count_ones();
            if size >= max_size && !has_cycle(sub) {
                if size > max_size {
                    max_size = size;
                    st.clear();
                }
                st.insert(sub);
            }
            let new_sub = (sub - 1) & mask1;
            if new_sub == mask1 {
                break;
            }
            sub = new_sub;
        }

        let mut ans = Vec::new();
        for &sub in &st {
            let mut cnt = vec![0i32; 26];
            for i in 0..26 {
                let a = (all & (1 << i)) != 0;
                let b = ((all ^ sub) & (1 << i)) != 0;
                cnt[i] = (a as i32) + (b as i32);
            }
            ans.push(cnt);
        }
        ans
    }
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let words: Vec<String> = lines.take(n).map(|s| s.to_string()).collect();

    let ans = Solution::supersequences(words);

    for cnt in ans {
        for &num in &cnt {
            print!("{} ", num);
        }
        println!();
    }
}