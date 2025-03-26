use std::io::{self, Read};
use std::cmp::Ordering;

// Define the Element struct to hold value and count
#[derive(Copy, Clone)]
struct Element {
    value: i32,
    count: i32,
}

// Implement Ord trait for Element to enable sorting
impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.count == other.count {
            other.value.cmp(&self.value)
        } else {
            other.count.cmp(&self.count)
        }
    }
}

// Implement PartialOrd trait for Element
impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implement PartialEq trait for Element
impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.value == other.value
    }
}

// Implement Eq trait for Element
impl Eq for Element {}

// Function to find X sum
fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
    let num_results = nums.len() - k as usize + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0; 51];
        for j in 0..k as usize {
            hash[nums[i + j] as usize] += 1;
        }

        let mut elements = Vec::new();
        for j in 0..51 {
            if hash[j] > 0 {
                elements.push(Element {
                    value: j as i32,
                    count: hash[j],
                });
            }
        }

        elements.sort();

        let sum = elements
            .iter()
            .take(x as usize)
            .map(|e| e.value * e.count)
            .sum();

        answer.push(sum);
    }

    answer
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    // Read k and x
    let mut kx = lines.next().unwrap().split_whitespace();
    let k: i32 = kx.next().unwrap().parse().unwrap();
    let x: i32 = kx.next().unwrap().parse().unwrap();

    // Read numsSize and nums
    let nums_size: usize = lines.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Calculate the result
    let answer = find_x_sum(&nums, k, x);

    // Print the result
    for num in answer {
        print!("{} ", num);
    }
    println!();

    Ok(())
}