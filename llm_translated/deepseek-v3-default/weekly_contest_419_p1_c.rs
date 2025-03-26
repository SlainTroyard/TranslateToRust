use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Element {
    value: i32,
    count: i32,
}

impl Element {
    fn new(value: i32, count: i32) -> Self {
        Self { value, count }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.value == other.value
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count == other.count {
            other.value.cmp(&self.value)
        } else {
            other.count.cmp(&self.count)
        }
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements = Vec::new();
        for (value, &count) in hash.iter().enumerate() {
            if count > 0 {
                elements.push(Element::new(value as i32, count));
            }
        }

        elements.sort();

        let elements_to_sum = if elements.len() < x {
            elements.len()
        } else {
            x
        };

        let sum: i32 = elements
            .iter()
            .take(elements_to_sum)
            .map(|e| e.value * e.count)
            .sum();

        answer.push(sum);
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let k_x_line = lines.next().unwrap().unwrap();
    let mut k_x_iter = k_x_line.split_whitespace();
    let k: usize = k_x_iter.next().unwrap().parse().unwrap();
    let x: usize = k_x_iter.next().unwrap().parse().unwrap();

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
    for num in answer {
        print!("{} ", num);
    }
    println!();
}