use std::io;

const INF: i32 = i32::MAX / 2;

fn max_difference(s: &[u8], k: i32) -> i32 {
    let mut ans = -INF;

    for x in 0..5 {
        for y in 0..5 {
            if x == y {
                continue;
            }
            let mut cur_s = [0i32; 5];
            let mut pre_s = [0i32; 5];
            let mut min_s = [[INF; 2]; 2];
            let mut left = 0;

            for i in 0..s.len() {
                let c = s[i] as usize;
                cur_s[c] += 1;

                let r = i + 1;
                // Shrink window while conditions are met
                while r - left >= k as usize && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let px = (pre_s[x] & 1) as usize;
                    let py = (pre_s[y] & 1) as usize;
                    min_s[px][py] = min_s[px][py].min(pre_s[x] - pre_s[y]);

                    let left_c = s[left] as usize;
                    pre_s[left_c] += 1;
                    left += 1;
                }

                // Update answer using current window and min_s
                let cx_parity = (cur_s[x] & 1) as usize;
                let cy_parity = (cur_s[y] & 1) as usize;
                let m_val = min_s[1 - cx_parity][cy_parity];
                ans = ans.max(cur_s[x] - cur_s[y] - m_val);
            }
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");

    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    if parts.len() != 2 {
        eprintln!("Error reading input");
        return;
    }

    let s_str = parts[0];
    let k_str = parts[1];
    let k: i32 = match k_str.parse() {
        Ok(k) => k,
        Err(_) => {
            eprintln!("Error reading input");
            return;
        }
    };

    let digits: Vec<u8> = s_str.chars()
        .map(|c| c.to_digit(10).expect("Input contains non-digit characters") as u8)
        .collect();

    let result = max_difference(&digits, k);
    println!("{}", result);
}