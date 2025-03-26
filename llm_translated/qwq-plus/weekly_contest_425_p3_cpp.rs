use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn min_array_sum(nums: &mut Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        nums.sort_unstable();

        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut candidates = BTreeSet::new();
        let mut swap_cnt = 0;

        // Phase 1
        let mut i = n - 1;
        while i >= m2 && op1 > 0 {
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;
            if op2 > 0 {
                nums[i] -= k;
                op2 -= 1;
            }
            i -= 1;
        }

        // Phase 2
        let mut j = m1;
        while j <= i && op2 > 0 {
            if k % 2 == 1 && nums[j] % 2 == 0 {
                candidates.insert(j);
            }
            nums[j] -= k;
            op2 -= 1;
            j += 1;
        }

        // Phase 3
        while i >= j && op1 > 0 {
            if k % 2 == 1 && nums[i] % 2 == 1 {
                swap_cnt += 1;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1 -= 1;
            i -= 1;
        }

        // Phase 4
        let mut arr: Vec<(i32, usize)> = (0..j)
            .map(|idx| (nums[idx], idx))
            .collect();
        arr.sort_unstable_by_key(|&(num, _)| num);

        while op1 > 0 && !arr.is_empty() {
            let (num, idx) = arr.pop().unwrap();
            nums[idx] = (num + 1) / 2;
            op1 -= 1;

            if candidates.contains(&idx) && swap_cnt > 0 {
                swap_cnt -= 1;
                nums[idx] -= 1;
            }
        }

        nums.iter().sum()
    }
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    let n: i32 = first_line[0].parse().unwrap();
    let k: i32 = first_line[1].parse().unwrap();
    let op1: i32 = first_line[2].parse().unwrap();
    let op2: i32 = first_line[3].parse().unwrap();

    let nums: Vec<i32> = lines[1]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut nums = nums;
    let solution = Solution;
    let result = solution.min_array_sum(&mut nums, k, op1, op2);
    println!("{}", result);
}