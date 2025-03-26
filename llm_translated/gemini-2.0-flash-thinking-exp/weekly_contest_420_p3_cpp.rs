use std::io;
use std::str::SplitWhitespace;

const MX: usize = 1_000_001;
static mut LPF: [usize; MX] = [0; MX];

// Initialize Least Prime Factor (LPF) array
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

struct Solution {}

impl Solution {
    fn min_operations(&self, nums: &mut Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                unsafe {
                    if nums[i] < MX as i32 && LPF[nums[i] as usize] != 0 {
                        nums[i] = LPF[nums[i] as usize] as i32;
                    } else if nums[i] >= MX as i32 {
                        // Handle case when nums[i] is out of LPF range.
                        // In original problem constraint, nums[i] is within 1e6, so this case should not happen given MX = 1000001.
                        // However for robustness, if it is out of range, we can't find LPF using precomputed table.
                        // For now, assume input is within range as per original problem.
                        // If needed, we can add prime factorization logic here for numbers larger than MX.
                        // But for now, let's follow the original code logic closely.
                        // Original code uses LPF array which is precomputed up to MX.
                        // If nums[i] > MX, and LPF[nums[i]] is accessed, it might cause out of bound access in C++ if not handled properly.
                        // However, given the problem context, it's likely nums[i] is within MX.
                        // Let's assume nums[i] is within MX for now, following original code logic.

                        // If LPF[nums[i]] == 0 even when nums[i] < MX, it means nums[i] is prime or 1.
                        // For prime number, LPF[prime] = prime if we initialize it properly.
                        // For 1, LPF[1] is not defined in typical prime factorization context, and usually we start LPF from 2.
                        // In the original code, LPF is initialized from 2. LPF[1] is likely 0 by default.
                        // If nums[i] becomes 1 after some operations, LPF[1] might be used, which is not properly initialized.
                        // Let's handle the case where LPF[nums[i]] == 0, although according to init_lpf, LPF[x] is always set for x >= 2.
                        if LPF[nums[i] as usize] == 0 {
                            // If LPF is not found (which shouldn't happen for nums[i] < MX and nums[i] >= 2 with correct init_lpf),
                            // we should probably handle it as no further reduction possible.
                            // In this problem context, it is unlikely to reach here given the init_lpf and constraints.
                            // For robustness, we can consider LPF[nums[i]] == nums[i] if not found in table (implying prime or error).
                            // But let's stick to original logic: nums[i] = LPF[nums[i]]. If LPF[nums[i]] == 0, then nums[i] becomes 0.
                            nums[i] = 0; // Or some other default behavior if LPF is not found, but original code sets LPF[j] = i in init.
                                        // So, LPF[x] should always be set for x >= 2. If nums[i] becomes 1, LPF[1] is 0. If nums[i] becomes 0, LPF[0] is 0.
                                        // If nums[i] becomes less than 2, LPF[nums[i]] access might be invalid or not meaningful.
                                        // But in the problem, nums are initially positive integers, and reduced by prime factors, so they should remain positive or become 1 eventually.
                                        // Let's assume nums[i] will always be >= 2 when LPF[nums[i]] is accessed in the loop, based on problem constraints and logic.
                        } else {
                            nums[i] = LPF[nums[i] as usize] as i32;
                        }

                    } else {
                        // nums[i] >= MX, handle if needed based on problem constraints.
                        // For now, assume nums[i] < MX as per original problem constraints.
                        // Or if nums[i] >= MX, and we are asked to find LPF, we would need factorization beyond precomputed table.
                        // For now, stick to precomputed LPF logic and assume nums[i] will be in range or become in range after reductions.
                        nums[i] = 1; // Or some default reduction if nums[i] is out of LPF range.
                    }
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

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn split_whitespace<'a>(s: &'a String) -> SplitWhitespace<'a> {
    s.split_whitespace()
}

fn parse_next<T: std::str::FromStr>(split_iter: &mut SplitWhitespace) -> T {
    split_iter.next().expect("Parse error").parse::<T>().ok().expect("Parse error")
}

fn main() {
    init_lpf(); // Initialize LPF table

    let n: usize = read_line().trim().parse().expect("Invalid input for n");
    let nums_line = read_line();
    let mut nums_split = split_whitespace(&nums_line);
    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..n {
        nums.push(parse_next::<i32>(&mut nums_split));
    }

    let sol = Solution {};
    println!("{}", sol.min_operations(&mut nums));
}