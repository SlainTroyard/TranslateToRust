use std::io;

const MX: usize = 1_000_001;

fn main() {
    let mut lpf: [usize; MX] = [0; MX];

    // Initialize LPF array
    for i in 2..MX {
        if lpf[i] == 0 {
            let mut j = i;
            while j < MX {
                if lpf[j] == 0 {
                    lpf[j] = i;
                }
                j += i;
            }
        }
    }

    let mut n_str = String::new();
    io::stdin().read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<usize> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let sol = Solution {};
    println!("{}", sol.min_operations(nums, &lpf));
}

struct Solution {}

impl Solution {
    fn min_operations(&self, mut nums: Vec<usize>, lpf: &[usize; MX]) -> i32 {
        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                nums[i] = lpf[nums[i]];
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}