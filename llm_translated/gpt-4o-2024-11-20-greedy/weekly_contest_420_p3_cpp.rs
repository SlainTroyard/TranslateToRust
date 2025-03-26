use std::io::{self, BufRead};

const MX: usize = 1_000_001;
static mut LPF: [usize; MX] = [0; MX];

// Initialize the LPF array (Least Prime Factor)
fn init_lpf() {
    unsafe {
        for i in 2..MX {
            if LPF[i] == 0 {
                for j in (i..MX).step_by(i) {
                    if LPF[j] == 0 {
                        LPF[j] = i;
                    }
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn min_operations(nums: &mut Vec<usize>) -> i32 {
        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                unsafe {
                    nums[i] = LPF[nums[i]];
                }
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    // Initialize LPF array
    init_lpf();

    // Read input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Ensure the input size matches the expected size
    assert_eq!(nums.len(), n);

    // Solve the problem
    let mut nums = nums;
    let solution = Solution;
    let result = solution.min_operations(&mut nums);

    // Output the result
    println!("{}", result);
}