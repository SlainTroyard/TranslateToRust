use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

// Frequency and value pair
#[derive(Clone, Copy, Debug)]
struct Pair {
    val: i32,   // Element value
    freq: i32,  // Frequency
}

impl Pair {
    fn new(val: i32, freq: i32) -> Self {
        Self { val, freq }
    }
}

// Compare function: prioritize by frequency (descending), then by value (descending)
fn compare_pairs(a: &Pair, b: &Pair) -> Ordering {
    match b.freq.cmp(&a.freq) {
        Ordering::Equal => b.val.cmp(&a.val),
        other => other,
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    let nums_size = nums.len();
    let result_size = nums_size - k + 1;
    let mut result = vec![0i64; result_size];
    
    // Use HashMap instead of a custom hash table implementation
    let mut freq_map: HashMap<i32, i32> = HashMap::with_capacity(k);
    
    // Process the first window
    for i in 0..k {
        *freq_map.entry(nums[i]).or_insert(0) += 1;
    }
    
    // Function to calculate the sum for the top x elements
    let calculate_sum = |freq_map: &HashMap<i32, i32>| -> i64 {
        // Collect all active elements
        let mut active_elements: Vec<Pair> = freq_map
            .iter()
            .map(|(&val, &freq)| Pair::new(val, freq))
            .filter(|p| p.freq > 0)
            .collect();
        
        // Sort active elements
        active_elements.sort_by(compare_pairs);
        
        // Calculate sum of top x elements
        let count_to_sum = std::cmp::min(active_elements.len(), x);
        let mut sum = 0i64;
        for i in 0..count_to_sum {
            sum += (active_elements[i].val as i64) * (active_elements[i].freq as i64);
        }
        sum
    };
    
    // Calculate the result for the first window
    result[0] = calculate_sum(&freq_map);
    
    // Slide the window
    for i in 1..=nums_size - k {
        // Remove the outgoing element
        let out_val = nums[i - 1];
        if let Some(freq) = freq_map.get_mut(&out_val) {
            *freq -= 1;
            if *freq == 0 {
                freq_map.remove(&out_val);
            }
        }
        
        // Add the incoming element
        let in_val = nums[i + k - 1];
        *freq_map.entry(in_val).or_insert(0) += 1;
        
        // Calculate the result for the current window
        result[i] = calculate_sum(&freq_map);
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and x
    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    
    // Read nums_size
    let nums_size: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Read nums array
    let line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate result
    let result = find_x_sum(&nums, k, x);
    
    // Print result
    let output: String = result.iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}