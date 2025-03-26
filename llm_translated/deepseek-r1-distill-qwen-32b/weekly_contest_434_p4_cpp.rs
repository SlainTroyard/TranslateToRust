use std::io;
use std::collections::HashSet;

struct Solution;

impl Solution {
    fn supersequences(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut all = 0;
        let mut mask2 = 0;
        let mut g = vec![vec![]; 26];
        for s in &words {
            let x = (s.chars().nth(0).unwrap() as u8 - b'a') as usize;
            let y = (s.chars().nth(1).unwrap() as u8 - b'a') as usize;
            all |= 1 << x;
            all |= 1 << y;
            if x == y {
                mask2 |= 1 << x;
            }
            g[x].push(y);
        }

        let mask1 = all ^ mask2;
        let mut max_size = 0;
        let mut st = HashSet::new();

        let mut sub = mask1;
        loop {
            let size = sub.count_ones() as usize;
            if size >= max_size {
                let mut cycle_found = false;
                let mut color = [0u8; 26];
                let mut stack = vec![];

                for i in 0..26 {
                    if (sub >> i & 1) != 0 && color[i] == 0 {
                        stack.push((i, false));
                        while let Some((x, visited)) = stack.pop() {
                            if visited {
                                color[x] = 2;
                                continue;
                            }
                            if color[x] == 1 {
                                cycle_found = true;
                                break;
                            }
                            color[x] = 1;
                            stack.push((x, true));
                            for &y in &g[x] {
                                if (sub >> y & 1) != 0 && color[y] != 2 {
                                    stack.push((y, false));
                                }
                            }
                        }
                        if cycle_found {
                            break;
                        }
                    }
                }

                if !cycle_found {
                    if size > max_size {
                        max_size = size;
                        st.clear();
                    }
                    st.insert(sub);
                }
            }

            sub = (sub - 1) & mask1;
            if sub == mask1 {
                break;
            }
        }

        let mut ans = vec![];
        for sub in st {
            let mut cnt = vec![0; 26];
            for i in 0..26 {
                cnt[i] = ((all >> i) & 1) as i32 + (((all ^ sub) >> i & 1) as i32);
            }
            ans.push(cnt);
        }

        ans
    }
}

fn read_input_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    let n = read_input_line().parse().unwrap();
    let words: Vec<String> = (0..n).map(|_| read_input_line()).collect();
    let sol = Solution;
    let ans = sol.supersequences(words);
    for cnt in ans {
        for i in 0..26 {
            print!("{} ", cnt[i]);
        }
        println!();
    }
}