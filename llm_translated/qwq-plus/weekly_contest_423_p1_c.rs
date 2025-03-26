use std::io;

fn has_increasing_subarrays(nums: &[i32], k: usize) -> bool {
    if k == 1 {
        return true;
    }
    for _j in 0..2 {
        for i in 0..=(nums.len() - 2 * k) {
            let s = i + k;
            let mut a = 0;
            for z in 0..(k - 1) {
                let first = nums[i + z] < nums[i + z + 1];
                let second = nums[s + z] < nums[s + z + 1];
                if first && second {
                    a += 1;
                }
            }
            if a == k - 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();

    let mut iter = numbers.iter();
    let n = *iter.next().expect("Missing n");
    let nums: Vec<i32> = iter
        .by_ref()
        .take(n as usize)
        .copied()
        .collect();
    let k = *iter.next().expect("Missing k") as usize;

    let result = has_increasing_subarrays(&nums, k);
    println!("{}", if result { "true" } else { "false" });
}