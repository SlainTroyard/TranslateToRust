use std::collections::BTreeSet;
use std::io;

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let op1: i32 = iter.next().unwrap().parse().unwrap();
    let op2: i32 = iter.next().unwrap().parse().unwrap();

    let mut nums = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for num in input.split_whitespace() {
        nums.push(num.parse::<i32>().unwrap());
    }

    // Call the function and print the result
    let result = min_array_sum(&mut nums, k, op1, op2);
    println!("{}", result);
}

fn min_array_sum(nums: &mut Vec<i32>, k: i32, mut op1: i32, mut op2: i32) -> i32 {
    let n = nums.len();
    nums.sort();

    // Find the boundaries for the three sections
    let m1 = nums.partition_point(|&x| x < k);
    let m2 = nums.partition_point(|&x| x < 2 * k - 1);

    let mut candidates = BTreeSet::new(); // To track indices of even numbers in the middle section
    let mut swap_cnt = 0;

    // Phase 1
    // Largest numbers, apply op1 then op2
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
    // Apply op2 in the middle section, from left to right
    let mut j = m1;
    while j <= i && op2 > 0 {
        if k % 2 == 1 && nums[j] % 2 == 0 {
            // k is odd and nums[j] is even, mark as a candidate for swapping
            candidates.insert(j);
        }
        nums[j] -= k;
        op2 -= 1;
        j += 1;
    }

    // Phase 3
    // Apply op1 to numbers in the middle section not already affected by op2
    while i >= j && op1 > 0 {
        if k % 2 == 1 && nums[i] % 2 == 1 {
            // k is odd and nums[i] is odd, increase swap count
            swap_cnt += 1;
        }
        nums[i] = (nums[i] + 1) / 2;
        op1 -= 1;
        i -= 1;
    }

    // Phase 4
    // Sort remaining untouched numbers and apply op1 greedily
    let mut arr: Vec<(i32, usize)> = nums[..j].iter().cloned().enumerate().map(|(idx, num)| (num, idx)).collect();
    arr.sort_by(|a, b| b.0.cmp(&a.0)); // Sort in descending order

    while op1 > 0 && !arr.is_empty() {
        let (num, idx) = arr.pop().unwrap();
        nums[idx] = (num + 1) / 2;
        op1 -= 1;

        if candidates.contains(&idx) && swap_cnt > 0 {
            // Handle the swap case
            swap_cnt -= 1;
            nums[idx] -= 1;
        }
    }

    // Return the sum of the modified nums
    nums.iter().sum()
}