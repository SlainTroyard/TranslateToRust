use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Copy)]
struct Pair {
    val: i32,
    freq: i32,
}

impl Pair {
    fn new(val: i32, freq: i32) -> Self {
        Self { val, freq }
    }
}

fn cmp(a: &Pair, b: &Pair) -> Ordering {
    if a.freq != b.freq {
        b.freq.cmp(&a.freq)
    } else {
        b.val.cmp(&a.val)
    }
}

fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let nums_size = nums.len();
    let return_size = nums_size - k as usize + 1;
    let mut result = vec![0; return_size];

    let mut hash_map = HashMap::new();
    let mut active_elements = Vec::with_capacity(k as usize);

    // Process the first window
    for &num in nums.iter().take(k as usize) {
        *hash_map.entry(num).or_insert(0) += 1;
    }

    // Collect all active elements
    for (&val, &freq) in hash_map.iter() {
        active_elements.push(Pair::new(val, freq));
    }

    // Sort active elements
    active_elements.sort_by(cmp);

    // Calculate the result for the first window
    let count_to_sum = active_elements.len().min(x as usize);
    let sum: i64 = active_elements.iter().take(count_to_sum).map(|p| p.val as i64 * p.freq as i64).sum();
    result[0] = sum;

    // Slide the window
    for i in 1..=nums_size - k as usize {
        let out_val = nums[i - 1];
        let in_val = nums[i + k as usize - 1];

        // Update the hash map - decrease the frequency of the outgoing element
        if let Some(freq) = hash_map.get_mut(&out_val) {
            *freq -= 1;
            if *freq == 0 {
                hash_map.remove(&out_val);
            }
        }

        // Update the hash map - increase the frequency of the incoming element
        *hash_map.entry(in_val).or_insert(0) += 1;

        // Reset active elements
        active_elements.clear();

        // Collect all active elements
        for (&val, &freq) in hash_map.iter() {
            active_elements.push(Pair::new(val, freq));
        }

        // Sort active elements
        active_elements.sort_by(cmp);

        // Calculate the result for the current window
        let count_to_sum = active_elements.len().min(x as usize);
        let sum: i64 = active_elements.iter().take(count_to_sum).map(|p| p.val as i64 * p.freq as i64).sum();
        result[i] = sum;
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = find_x_sum(&nums, k, x);

    for &res in result.iter() {
        print!("{} ", res);
    }
    println!();
}