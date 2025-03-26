use std::io;

fn max_difference(s: &str, k: i32) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;
    let len = s.len();
    
    for x in 0..5 {
        for y in 0..5 {
            if x == y {
                continue;
            }
            
            let mut cur_s = [0; 5];
            let mut pre_s = [0; 5];
            let mut min_s = [[INF; 2]; 2];
            let mut left = 0;
            
            for i in 0..len {
                let digit = s[i].to_digit(10).unwrap() as i32;
                cur_s[digit as usize] += 1;
                
                let r = i + 1;
                
                while (r - left) as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let px = (pre_s[x] % 2) as usize;
                    let py = (pre_s[y] % 2) as usize;
                    min_s[px][py] = std::cmp::min(min_s[px][py], pre_s[x] - pre_s[y]);
                    
                    let left_digit = s[left].to_digit(10).unwrap() as i32;
                    pre_s[left_digit as usize] += 1;
                    left += 1;
                }
                
                let cx = (cur_s[x] % 2) as usize;
                let cy = (cur_s[y] % 2) as usize;
                let opposite_cx = 1 - cx;
                let min_val = min_s[opposite_cx][cy];
                ans = std::cmp::max(ans, cur_s[x] - cur_s[y] - min_val);
            }
        }
    }
    
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    if parts.len() < 2 {
        eprintln!("Error reading input");
        return;
    }
    
    let s = parts[0];
    let k = parts[1].parse::<i32>().expect("Invalid integer");
    
    let result = max_difference(s, k);
    println!("{}", result);
}