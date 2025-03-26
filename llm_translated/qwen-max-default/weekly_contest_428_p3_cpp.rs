use std::io::{self, BufRead, Write};

fn beautiful_splits(nums: &Vec<i32>) -> i32 {
    let n = nums.len();
    // lcp[i][j] represents the Longest Common Prefix (LCP) between s[i..] and s[j..]
    let mut lcp = vec![vec![0; n + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (i..n).rev() {
            if nums[i] == nums[j] {
                lcp[i][j] = lcp[i + 1][j + 1] + 1;
            }
        }
    }

    let mut ans = 0;
    for i in 1..n - 1 {
        for j in i + 1..n {
            // Check if the split satisfies the beautiful condition
            if (i <= j - i && lcp[0][i] >= i as usize) || lcp[i][j] >= j - i {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // Read the size of the array
    stdin.lock().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().expect("Invalid input");
    buffer.clear();

    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        stdin.lock().read_line(&mut buffer).unwrap();
        nums.push(buffer.trim().parse::<i32>().expect("Invalid input"));
        buffer.clear();
    }

    // Calculate and print the result
    let result = beautiful_splits(&nums);
    writeln!(stdout, "{}", result).unwrap();
}