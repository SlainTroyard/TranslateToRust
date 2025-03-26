use std::collections::BTreeSet;
use std::io::Read;

struct Solution;

impl Solution {
    fn min_array_sum(nums: &mut Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
        let n = nums.len();
        nums.sort();

        let m1 = nums.partition_point(|&x| x < k);
        let m2 = nums.partition_point(|&x| x < 2 * k - 1);

        let mut candidates = BTreeSet::new();
        let mut swap_cnt = 0;

        // Phase 1: Apply op1 and op2 to the largest elements
        let mut i = (n as isize) - 1;
        while i >= m2 as isize && op1 > 0 {
            let idx = i as usize;
            nums[idx] = (nums[idx] + 1) / 2;
            op1 -= 1;
            if op2 > 0 {
                nums[idx] -= k;
                op2 -= 1;
            }
            i -= 1;
        }

        // Phase 2: Apply op2 to the middle section
        let mut j = m1;
        while (j as isize) <= i && op2 > 0 {
            if k % 2 == 1 && nums[j] % 2 == 0 {
                candidates.insert(j);
            }
            nums[j] -= k;
            op2 -= 1;
            j += 1;
        }

        // Phase 3: Apply op1 to remaining middle section
        while i >= j as isize && op1 > 0 {
            let idx = i as usize;
            if k % 2 == 1 && nums[idx] % 2 == 1 {
                swap_cnt += 1;
            }
            nums[idx] = (nums[idx] + 1) / 2;
            op1 -= 1;
            i -= 1;
        }

        // Phase 4: Greedily apply remaining op1
        let mut arr = Vec::new();
        for idx in 0..j {
            arr.push((nums[idx], idx));
        }
        arr.sort();

        while op1 > 0 && !arr.is_empty() {
            if let Some((num, idx)) = arr.pop() {
                nums[idx] = (num + 1) / 2;
                op1 -= 1;

                if candidates.contains(&idx) && swap_cnt > 0 {
                    swap_cnt -= 1;
                    nums[idx] -= 1;
                }
            }
        }

        nums.iter().sum()
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = tokens.next().unwrap() as usize;
    let k = tokens.next().unwrap();
    let op1 = tokens.next().unwrap();
    let op2 = tokens.next().unwrap();

    let mut nums: Vec<i32> = tokens.take(n).collect();

    assert_eq!(nums.len(), n, "Incorrect number of elements in the array");

    let result = Solution::min_array_sum(&mut nums, k, op1, op2);
    println!("{}", result);
}