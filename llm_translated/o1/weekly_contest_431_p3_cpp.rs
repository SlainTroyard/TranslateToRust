use std::io;

/// Reads a single line from stdin.
fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

/// The solution struct, mirroring the C++ class.
struct Solution;

impl Solution {
    /// Translated method that computes the maximum coins.
    fn maximum_coins(&self, nums: &mut Vec<Vec<i32>>, k: i32) -> i64 {
        // Sort by the first element (nums[i][0]) in ascending order
        nums.sort_by(|a, b| a[0].cmp(&b[0]));

        let n = nums.len();
        let mut ans: i64 = 0;

        // Build prefix sum array where prefix[i] = sum of coins from intervals [0..i-1]
        // The coin count for a single interval is (end - start + 1) * coins
        let mut presum = vec![0i64; n + 1];
        for i in 1..=n {
            let start = nums[i - 1][0] as i64;
            let end = nums[i - 1][1] as i64;
            let c = nums[i - 1][2] as i64;
            presum[i] = presum[i - 1] + (end - start + 1) * c;
        }

        // First pass: move a sliding window from left to right
        let mut left = 0usize;
        let mut right = 0usize;
        while right < n && left < n {
            // Shrink from the left while the interval length exceeds k
            while left < n && (nums[right][1] - nums[left][0] + 1) as i64 > k as i64 {
                let tamp = k as i64 - (nums[right][0] - nums[left][0]) as i64;
                let val = tamp * nums[right][2] as i64 + (presum[right] - presum[left]);
                if val > ans {
                    ans = val;
                }
                left += 1;
            }
            if left >= n {
                break;
            }
            // Update the answer using the prefix sums
            let val = presum[right + 1] - presum[left];
            if val > ans {
                ans = val;
            }
            right += 1;
        }

        // Second pass: move a sliding window from right to left
        let mut left_i: isize = n as isize - 1;
        let mut right_i: isize = n as isize - 1;
        while right_i >= 0 && left_i >= 0 {
            while right_i >= 0 && (nums[right_i as usize][1] - nums[left_i as usize][0] + 1) as i64 > k as i64 {
                let tamp = k as i64 - (nums[right_i as usize][1] - nums[left_i as usize][1]) as i64;
                let val = tamp * (nums[left_i as usize][2] as i64)
                    + (presum[(right_i + 1) as usize] - presum[(left_i + 1) as usize]);
                if val > ans {
                    ans = val;
                }
                right_i -= 1;
            }
            if right_i < 0 {
                break;
            }
            let val = presum[(right_i + 1) as usize] - presum[left_i as usize];
            if val > ans {
                ans = val;
            }
            left_i -= 1;
        }

        ans
    }
}

fn main() {
    // Read n and K
    let line = read_line().expect("Failed to read line for n, K");
    let mut parts = line.trim().split_whitespace();
    let n = parts.next().unwrap().parse::<usize>().expect("Failed to parse n");
    let k = parts.next().unwrap().parse::<i32>().expect("Failed to parse K");

    // Read the intervals
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        let line = read_line().expect("Failed to read line for intervals");
        let mut values = line.trim().split_whitespace();
        let s = values.next().unwrap().parse::<i32>().expect("Failed to parse start");
        let e = values.next().unwrap().parse::<i32>().expect("Failed to parse end");
        let c = values.next().unwrap().parse::<i32>().expect("Failed to parse coins");
        vec.push(vec![s, e, c]);
    }

    // Compute the answer and print it
    let sol = Solution;
    let ans = sol.maximum_coins(&mut vec, k);
    println!("{}", ans);
}