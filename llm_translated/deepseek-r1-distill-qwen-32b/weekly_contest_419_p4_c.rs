use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Pair {
    val: i32,
    freq: i32,
}

fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let n = nums.len() as i32;
    let mut result = Vec::with_capacity((n - k + 1) as usize);
    
    if k == 0 || n < k {
        return result;
    }
    
    let mut freq_map = HashMap::new();
    
    // Initialize the first window
    for i in 0..k as usize {
        let val = nums[i];
        *freq_map.entry(val).or_insert(0) += 1;
    }
    
    // Function to compute the sum for current frequency map
    let compute_sum = |freq_map: &HashMap<i32, i32>, x: i32| -> i64 {
        let mut elements: Vec<_> = freq_map.iter().map(|(&val, &freq)| Pair { val, freq }).collect();
        elements.sort_by(|a, b| {
            if a.freq != b.freq {
                b.freq.cmp(&a.freq)
            } else {
                b.val.cmp(&a.val)
            }
        });
        
        let sum_count = std::cmp::min(x as usize, elements.len());
        elements.iter().take(sum_count)
            .fold(0, |acc, pair| acc + (pair.val as i64) * (pair.freq as i64))
    };
    
    // Compute sum for the first window
    let sum = compute_sum(&freq_map, x);
    result.push(sum);
    
    // Slide the window
    for i in 1..(n - k + 1) as usize {
        let out_val = nums[i - 1];
        let in_val = nums[i + k as usize - 1];
        
        // Remove outgoing value
        if let Some(count) = freq_map.get_mut(&out_val) {
            *count -= 1;
            if *count == 0 {
                freq_map.remove(&out_val);
            }
        }
        
        // Add incoming value
        *freq_map.entry(in_val).or_insert(0) += 1;
        
        // Compute sum for current window
        let sum = compute_sum(&freq_map, x);
        result.push(sum);
    }
    
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let x: i32 = parts.next().unwrap().parse().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let result = find_x_sum(&nums, k, x);
    
    for num in result {
        print!("{} ", num);
    }
    println!();
}