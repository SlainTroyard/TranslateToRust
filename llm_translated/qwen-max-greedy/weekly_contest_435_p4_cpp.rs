use std::io::{self, BufRead, Write};

fn max_difference(s: &str, k: usize) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut ans = -INF;

    for x in 0..5 {
        for y in 0..5 {
            if y == x {
                continue;
            }
            let mut cur_s = [0; 5];
            let mut pre_s = [0; 5];
            let mut min_s = [[INF, INF], [INF, INF]];
            let mut left = 0;

            for (i, ch) in s.chars().enumerate() {
                let digit = ch.to_digit(10).unwrap() as usize;
                cur_s[digit] += 1;

                let r = i + 1;
                while r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                    let p = &mut min_s[pre_s[x] & 1][pre_s[y] & 1];
                    *p = (*p).min(pre_s[x] - pre_s[y]);
                    pre_s[s.chars().nth(left).unwrap().to_digit(10).unwrap() as usize] += 1;
                    left += 1;
                }

                ans = ans.max(cur_s[x] - cur_s[y] - min_s[cur_s[x] & 1 ^ 1][cur_s[y] & 1]);
            }
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the input string and integer
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");
    let input: Vec<&str> = buffer.trim().split_whitespace().collect();
    let s = input[0];
    let k: usize = input[1].parse().expect("Failed to parse k");

    // Create a solution instance and compute the result
    let sol = max_difference(s, k);

    // Output the result
    writeln!(stdout, "{}", sol).expect("Failed to write to stdout");
}