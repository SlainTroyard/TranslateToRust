struct Solution;

impl Solution {
    fn max_difference(s: String, k: i32) -> i32 {
        let inf = i32::MAX / 2;
        let mut ans = -inf;
        let digits: Vec<usize> = s
            .chars()
            .map(|c| {
                let d = c.to_digit(10).expect("Invalid digit");
                assert!(d < 5, "Digit must be between 0 and 4");
                d as usize
            })
            .collect();

        for x in 0..5 {
            for y in 0..5 {
                if x == y {
                    continue;
                }
                let mut cur_s = [0; 5];
                let mut pre_s = [0; 5];
                let mut min_s = [[inf; 2]; 2];
                let mut left = 0;

                for (i, &digit) in digits.iter().enumerate() {
                    cur_s[digit] += 1;
                    let r = i + 1;

                    // Process the sliding window to update min_s
                    while (r - left) as i32 >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y] {
                        let px = pre_s[x];
                        let py = pre_s[y];
                        let parity_x = (px % 2) as usize;
                        let parity_y = (py % 2) as usize;
                        min_s[parity_x][parity_y] = min_s[parity_x][parity_y].min(px - py);

                        let left_digit = digits[left];
                        pre_s[left_digit] += 1;
                        left += 1;
                    }

                    // Calculate the current potential maximum difference
                    let current_x = cur_s[x];
                    let current_y = cur_s[y];
                    let target_parity_x = ((current_x & 1) ^ 1) as usize;
                    let parity_y = (current_y & 1) as usize;
                    let val = current_x - current_y - min_s[target_parity_x][parity_y];
                    ans = ans.max(val);
                }
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let s = parts[0].to_string();
    let k: i32 = parts[1].parse().expect("Invalid k value");

    println!("{}", Solution::max_difference(s, k));
}