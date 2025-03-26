use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Element {
    value: i32,
    count: i32,
}

// Comparator function for sorting elements
fn cmp(a: &Element, b: &Element) -> Ordering {
    if a.count == b.count {
        b.value.cmp(&a.value) // Descending order by value if counts are equal
    } else {
        b.count.cmp(&a.count) // Descending order by count
    }
}

// Function to compute the x-sum for each subarray of size k
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0; 51]; // Frequency array for numbers in range [0, 50]

        // Count frequencies of elements in the current subarray
        for j in 0..k {
            hash[nums[i + j] as usize] += 1;
        }

        // Collect elements with their counts
        let mut elements: Vec<Element> = hash
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count > 0)
            .map(|(value, &count)| Element {
                value: value as i32,
                count,
            })
            .collect();

        // Sort elements by count (descending), then by value (descending)
        elements.sort_by(cmp);

        // Calculate the x-sum
        let mut sum = 0;
        let elements_to_sum = elements.len().min(x);
        for j in 0..elements_to_sum {
            sum += elements[j].value * elements[j].count;
        }

        answer.push(sum);
    }

    answer
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read k and x
    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let k: usize = first_iter.next().unwrap().parse().unwrap();
    let x: usize = first_iter.next().unwrap().parse().unwrap();

    // Read numsSize
    let second_line = lines.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    // Read nums array
    let third_line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Ensure nums_size matches the actual size of nums
    assert_eq!(nums.len(), nums_size);

    // Compute the result
    let result = find_x_sum(&nums, k, x);

    // Print the result
    for (i, value) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", value);
    }
    println!();
}