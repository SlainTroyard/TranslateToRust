use std::io::{self, BufRead};
use std::cmp::Ordering;

/// Element struct to store value and its count for sorting
#[derive(Debug, Clone, Copy)]
struct Element {
    value: i32,
    count: i32,
}

/// Function to find the sum of the most frequent x elements in each window of size k
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let nums_size = nums.len();
    let num_results = nums_size - k + 1;
    let mut answer = vec![0; num_results];

    for i in 0..num_results {
        // Count frequency of each number in the current window
        let mut hash = vec![0; 51]; // As per the constraints, nums[i] is between 0 and 50
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        // Create array of elements with their counts
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

        // Calculate the sum of the top x elements
        let elements_to_sum = std::cmp::min(elements.len(), x);
        let mut sum = 0;
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
    
    // Read k and x from the first line
    let first_line = lines.next().unwrap()?;
    let mut parts = first_line.split_whitespace();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let x: usize = parts.next().unwrap().parse().unwrap();
    
    // Read numsSize from the second line
    let nums_size: usize = lines.next().unwrap()?.parse().unwrap();
    
    // Read nums from the third line
    let nums_line = lines.next().unwrap()?;
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    // Calculate the result
    let answer = find_x_sum(&nums, k, x);
    
    // Print the result
    for (i, &val) in answer.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
    
    Ok(())
}