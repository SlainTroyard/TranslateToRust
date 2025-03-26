use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Element {
    value: i32,
    count: i32,
}

// Comparator function for sorting elements
fn cmp(a: &Element, b: &Element) -> Ordering {
    if a.count == b.count {
        b.value.cmp(&a.value)
    } else {
        b.count.cmp(&a.count)
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = vec![0; num_results];

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements = Vec::new();
        for (value, &count) in hash.iter().enumerate() {
            if count > 0 {
                elements.push(Element {
                    value: value as i32,
                    count,
                });
            }
        }

        elements.sort_by(cmp);

        let elements_to_sum = if elements.len() < x { elements.len() } else { x };
        let sum: i32 = elements.iter().take(elements_to_sum).map(|e| e.value * e.count).sum();
        answer[i] = sum;
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: usize = parts.next().unwrap().parse().unwrap();
    let x: usize = parts.next().unwrap().parse().unwrap();

    // Read numsSize
    let nums_size: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Read nums
    let nums_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = nums_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Compute the answer
    let answer = find_x_sum(&nums, k, x);

    // Print the answer
    for val in answer {
        print!("{} ", val);
    }
    println!();
}