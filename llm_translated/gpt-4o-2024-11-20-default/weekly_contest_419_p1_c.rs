use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Element {
    value: i32,
    count: i32,
}

// Function to sort elements as per the logic in the C code
fn compare_elements(a: &Element, b: &Element) -> Ordering {
    if a.count == b.count {
        // Descending by value
        b.value.cmp(&a.value)
    } else {
        // Descending by count
        b.count.cmp(&a.count)
    }
}

// Function to calculate the required sums
fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = nums.len() - k + 1;
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        // Create a frequency map for the current sliding window
        let mut hash = HashMap::new();
        for j in 0..k {
            *hash.entry(nums[i + j]).or_insert(0) += 1;
        }

        // Create elements from the frequency map
        let mut elements: Vec<Element> = hash
            .iter()
            .map(|(&value, &count)| Element { value, count })
            .collect();

        // Sort elements as per the C code comparison logic
        elements.sort_by(compare_elements);

        // Compute sum of top `x` elements
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
    // Input handling
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums_size: usize = input.trim().parse().expect("Invalid numsSize");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k_x: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid k or x"))
        .collect();
    let k = k_x[0];
    let x = k_x[1];

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid number in nums"))
        .collect();

    // Ensure nums_size matches the actual size of nums
    assert_eq!(nums.len(), nums_size, "Mismatch in numsSize");

    // Call the function
    let answer = find_x_sum(&nums, k, x);

    // Output the result
    for value in answer {
        print!("{} ", value);
    }
    println!();
}