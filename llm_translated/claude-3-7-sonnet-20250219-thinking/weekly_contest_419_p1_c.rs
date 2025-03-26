use std::cmp::Ordering;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Element {
    value: i32,
    count: i32,
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by count (descending)
        match other.count.cmp(&self.count) {
            Ordering::Equal => {
                // If counts are equal, compare by value (descending)
                other.value.cmp(&self.value)
            }
            other_ordering => other_ordering,
        }
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let nums_size = nums.len();
    let num_results = nums_size - k + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0; 51]; // Count frequency of each number (0-50)
        
        // Count occurrences in the current window
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        // Create elements from non-zero counts
        let mut elements = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element {
                    value: j as i32,
                    count: hash[j],
                });
            }
        }

        // Sort elements by count (and value if tied) in descending order
        elements.sort();

        // Calculate sum of top x elements
        let elements_to_sum = x.min(elements.len());
        let sum: i32 = elements.iter()
            .take(elements_to_sum)
            .map(|e| e.value * e.count)
            .sum();

        answer.push(sum);
    }

    answer
}

fn main() {
    // Read k and x
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let k: usize = iter.next().unwrap().parse().expect("Failed to parse k");
    let x: usize = iter.next().unwrap().parse().expect("Failed to parse x");
    
    // Read nums size
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Failed to parse nums_size");
    
    // Read nums
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    // Call find_x_sum and print results
    let answer = find_x_sum(&nums, k, x);
    
    // Print results with space between numbers
    for (i, &val) in answer.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", val);
    }
    println!();
}