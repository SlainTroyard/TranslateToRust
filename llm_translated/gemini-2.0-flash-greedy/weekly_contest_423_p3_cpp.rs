use std::collections::HashMap;
use std::io;

fn main() {
    // Reading input
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Invalid input");

    let mut nums_str = String::new();
    io::stdin().read_line(&mut nums_str).expect("Failed to read line");
    let nums: Vec<i32> = nums_str
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Create an object of Solution (no need in Rust)

    // Call the method and get the result
    let result = sum_of_good_subsequences(&nums);

    // Output the result
    println!("{}", result);
}

fn sum_of_good_subsequences(nums: &Vec<i32>) -> i32 {
    let mod_val: i64 = 1_000_000_007;

    let mut cnt: HashMap<i32, i64> = HashMap::new();
    let mut sum: HashMap<i32, i64> = HashMap::new();

    // Loop through all numbers in the nums array
    for &i in nums {
        // Update cnt[i] by considering subsequences from i-1, i, and i+1
        let prev_cnt = *cnt.get(&(i - 1)).unwrap_or(&0);
        let next_cnt = *cnt.get(&(i + 1)).unwrap_or(&0);

        let new_cnt = (prev_cnt + next_cnt + 1) % mod_val;

        *cnt.entry(i).or_insert(0) += new_cnt;
        *cnt.entry(i).or_insert(0) %= mod_val;


        // Update sum[i] by considering subsequences from i-1, i, and i+1
        let prev_sum = *sum.get(&(i - 1)).unwrap_or(&0);
        let next_sum = *sum.get(&(i + 1)).unwrap_or(&0);

        *sum.entry(i).or_insert(0) += (prev_sum + next_sum) % mod_val;
        *sum.entry(i).or_insert(0) %= mod_val;

        // Add the contribution of the subsequences that end at i
        let contribution = ((prev_cnt + next_cnt + 1) % mod_val) * (i as i64) % mod_val;

        *sum.entry(i).or_insert(0) += contribution;
        *sum.entry(i).or_insert(0) %= mod_val;
    }

    // Calculate the final result by summing all the values in sum
    let mut res: i64 = 0;
    for &val in sum.values() {
        res += val;
        res %= mod_val;
    }

    res as i32
}