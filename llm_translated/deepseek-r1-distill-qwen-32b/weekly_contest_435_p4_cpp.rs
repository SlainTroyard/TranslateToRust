use std::io;

const INF: i32 = i32::MAX / 2;

struct Solution {}

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let mut ans = -INF;
        let s = s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>();
        let k = k as usize;
        
        for x in 0..5 {
            for y in 0..5 {
                if x == y {
                    continue;
                }
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[INF; 2]; 2];
                let mut left = 0;
                
                for i in 0..s.len() {
                    cur_s[s[i]] += 1;
                    let r = i + 1;
                    
                    while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let p_x = pre_s[x] & 1;
                        let p_y = pre_s[y] & 1;
                        min_s[p_x][p_y] = std::cmp::min(min_s[p_x][p_y], pre_s[x] - pre_s[y]);
                        pre_s[s[left]] += 1;
                        left += 1;
                    }
                    
                    let c_x = cur_s[x] & 1;
                    let c_y = cur_s[y] & 1;
                    ans = std::cmp::max(ans, cur_s[x] - cur_s[y] - min_s[c_x ^ 1][c_y]);
                }
            }
        }
        
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split(' ').collect();
    let s = parts[0].to_string();
    let k = parts[1].parse::<i32>().unwrap();
    
    let sol = Solution {};
    println!("{}", sol.max_difference(s, k));
}