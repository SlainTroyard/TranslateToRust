use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

// Frequency and value pair
#[derive(Clone, Copy, PartialEq, Eq)]
struct Pair {
    val: i32,   // Element value
    freq: i32,  // Frequency
}

impl Pair {
    fn new(val: i32, freq: i32) -> Self {
        Pair { val, freq }
    }
}

// Custom ordering: first by frequency (descending), then by value (descending)
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.freq.cmp(&self.freq) {
            Ordering::Equal => other.val.cmp(&self.val),
            other_ordering => other_ordering,
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    let nums_size = nums.len();
    let result_size = nums_size - k + 1;
    let mut result = vec![0i64; result_size];
    
    // Use a HashMap instead of a custom hash table implementation
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    
    // Process the first window
    for i in 0..k {
        *freq_map.entry(nums[i]).or_insert(0) += 1;
    }
    
    // Calculate result for the first window
    let mut active_elements: Vec<Pair> = freq_map
        .iter()
        .map(|(&val, &freq)| Pair::new(val, freq))
        .collect();
    
    active_elements.sort_unstable();
    
    let count_to_sum = std::cmp::min(active_elements.len(), x);
    let mut sum = 0i64;
    for i in 0..count_to_sum {
        sum += i64::from(active_elements[i].val) * i64::from(active_elements[i].freq);
    }
    result[0] = sum;
    
    // Slide the window
    for i in 1..=nums_size - k {
        let out_val = nums[i - 1];    // Element leaving the window
        let in_val = nums[i + k - 1]; // Element entering the window
        
        // Update frequency map - decrease frequency of outgoing element
        if let Some(freq) = freq_map.get_mut(&out_val) {
            *freq -= 1;
            if *freq == 0 {
                freq_map.remove(&out_val);
            }
        }
        
        // Update frequency map - increase frequency of incoming element
        *freq_map.entry(in_val).or_insert(0) += 1;
        
        // Recollect all active elements
        active_elements = freq_map
            .iter()
            .map(|(&val, &freq)| Pair::new(val, freq))
            .collect();
        
        active_elements.sort_unstable();
        
        // Calculate result for current window
        let count_to_sum = std::cmp::min(active_elements.len(), x);
        let mut sum = 0i64;
        for j in 0..count_to_sum {
            sum += i64::from(active_elements[j].val) * i64::from(active_elements[j].freq);
        }
        result[i] = sum;
    }
    
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and x
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    
    // Read nums_size
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate result
    let result = find_x_sum(&nums, k, x);
    
    // Print result
    for (i, val) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}