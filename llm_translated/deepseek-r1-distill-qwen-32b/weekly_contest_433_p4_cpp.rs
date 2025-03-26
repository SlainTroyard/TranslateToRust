use std::io;

struct Solution;

impl Solution {
    fn min_max_subarray_sum(mut nums: Vec<i32>, k: i32) -> i64 {
        // Helper function to compute the sum of subarray minimums
        fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
            let count = |m: i32| -> i64 {
                if m > k {
                    ((2 * m - k + 1) * k) / 2
                } else {
                    ((m + 1) * m) / 2
                }
            };

            let mut stack = vec![-1];
            let mut res = 0;

            for r in 0..nums.len() {
                let current_num = nums[r];
                while stack.len() > 1 && nums[(stack[stack.len() - 1] + 1) as usize] >= current_num {
                    let i = stack.pop().unwrap();
                    let l = stack[stack.len() - 1];
                    let m1 = r as i32 - l - 1;
                    let m2 = i - l - 1;
                    let m3 = r as i32 - i - 1;
                    let cnt = count(m1) - count(m2) - count(m3);
                    res += nums[(i + 1) as usize] as i64 * cnt;
                }
                stack.push(r as i32);
            }

            res
        }

        // Create a copy of nums with INT_MIN / 2 appended
        let mut temp = nums.clone();
        temp.push(i32::MIN / 2);
        let sum1 = sum_subarray_mins(&temp, k);

        // Create a new vector where each element except the last is negated
        let mut temp2 = temp.clone();
        for x in &mut temp2[..temp2.len() - 1] {
            *x = -*x;
        }
        let sum2 = sum_subarray_mins(&temp2, k);

        sum1 - sum2
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let n = tokens.next().unwrap().parse::<i32>().unwrap();
    let k = tokens.next().unwrap().parse::<i32>().unwrap();
    let mut nums = Vec::new();
    for _ in 0..n {
        nums.push(tokens.next().unwrap().parse::<i32>().unwrap());
    }
    let sol = Solution;
    println!("{}", sol.min_max_subarray_sum(nums, k));
}