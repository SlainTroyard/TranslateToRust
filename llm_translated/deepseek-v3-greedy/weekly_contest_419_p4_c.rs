use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

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

    let mut hash_table: HashMap<i32, i32> = HashMap::new();
    let mut active_elements = Vec::with_capacity(k as usize);

    // Process the first window
    for &num in nums.iter().take(k as usize) {
        *hash_table.entry(num).or_insert(0) += 1;
    }

    // Collect all active elements
    for (&val, &freq) in &hash_table {
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

        // Update hash table - decrease frequency of the element going out
        if let Some(freq) = hash_table.get_mut(&out_val) {
            *freq -= 1;
            if *freq == 0 {
                hash_table.remove(&out_val);
            }
        }

        // Update hash table - increase frequency of the element coming in
        *hash_table.entry(in_val).or_insert(0) += 1;

        // Reset active elements
        active_elements.clear();

        // Collect all active elements
        for (&val, &freq) in &hash_table {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let k: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let nums_size: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        nums.push(num);
    }

    let result = find_x_sum(&nums, k, x);

    for res in result {
        print!("{} ", res);
    }
    println!();
}