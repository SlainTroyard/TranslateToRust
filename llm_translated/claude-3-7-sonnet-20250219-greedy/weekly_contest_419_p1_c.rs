use std::cmp::Ordering;
use std::io::{self, BufRead};

// Element struct to store value and its count
#[derive(Debug, Clone, Copy)]
struct Element {
    value: i32,
    count: i32,
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let nums_size = nums.len();
    let num_results = nums_size - k + 1;
    let mut answer = vec![0; num_results];

    for i in 0..num_results {
        // Count occurrences of each number in the current window
        let mut hash = vec![0; 51]; // Assuming numbers are in range [0, 50]
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        // Create elements array with non-zero counts
        let mut elements = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element {
                    value: j as i32,
                    count: hash[j],
                });
            }
        }

        // Sort elements by count (descending) and then by value (descending)
        elements.sort_by(|a, b| {
            match b.count.cmp(&a.count) {
                Ordering::Equal => b.value.cmp(&a.value),
                other => other,
            }
        });

        // Calculate sum of the top x elements (considering their counts)
        let mut sum = 0;
        let elements_to_sum = std::cmp::min(elements.len(), x);
        for j in 0..elements_to_sum {
            sum += elements[j].value * elements[j].count;
        }

        answer[i] = sum;
    }

    answer
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read k and x
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    
    // Read nums size
    let nums_size: usize = lines.next().unwrap()?.trim().parse().unwrap();
    
    // Read nums array
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate the answer
    let answer = find_x_sum(&nums, k, x);
    
    // Print the answer
    for (i, val) in answer.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}