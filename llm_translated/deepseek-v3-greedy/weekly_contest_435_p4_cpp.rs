use std::cmp::{max, min};
use std::io::{self, BufRead};

const INF: i32 = i32::MAX / 2;

fn max_difference(s: &str, k: i32) -> i32 {
    let mut ans = -INF;
    let s_bytes = s.as_bytes();
    
    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }
            
            let mut cur_s = [0; 5];
            let mut pre_s = [0; 5];
            let mut min_s = [[INF, INF], [INF, INF]];
            let mut left = 0;
            
            for i in 0..s_bytes.len() {
                cur_s[(s_bytes[i] - b'0') as usize] += 1;
                let r = i + 1;
                
                while (r - left) as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[(pre_s[x] & 1) as usize][(pre_s[y] & 1) as usize];
                    *p = min(*p, pre_s[x] - pre_s[y]);
                    pre_s[(s_bytes[left] - b'0') as usize] += 1;
                    left += 1;
                }
                
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1 ^ 1) as usize][(cur_s[y] & 1) as usize]);
            }
        }
    }
    
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let s = lines.next().unwrap().unwrap();
    let k: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    println!("{}", max_difference(&s, k));
}