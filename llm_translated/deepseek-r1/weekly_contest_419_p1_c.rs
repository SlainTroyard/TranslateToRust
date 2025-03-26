use std::io::{self, Read};

#[derive(Debug)]
struct Element {
    value: i32,
    count: i32,
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    let num_results = if nums.len() >= k {
        nums.len() - k + 1
    } else {
        0
    };
    let mut answer = Vec::with_capacity(num_results);

    for i in 0..num_results {
        let mut hash = [0i32; 51];
        // Populate frequency hash for the current window
        for j in 0..k {
            let num = nums[i + j];
            hash[num as usize] += 1;
        }

        // Collect elements with count > 0
        let mut elements = Vec::new();
        for (value, &count) in hash.iter().enumerate() {
            if count > 0 {
                elements.push(Element {
                    value: value as i32,
                    count,
                });
            }
        }

        // Sort elements by descending count, then descending value
        elements.sort_by(|a, b| {
            b.count.cmp(&a.count)
                .then_with(|| b.value.cmp(&a.value))
        });

        // Calculate sum of top x elements
        let elements_to_sum = elements.len().min(x);
        let sum: i32 = elements.iter()
            .take(elements_to_sum)
            .map(|e| e.value * e.count)
            .sum();

        answer.push(sum);
    }

    answer
}

fn main() {
    // Read entire input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut tokens = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer input"));

    // Parse k and x from first line
    let k = tokens.next().expect("Missing k") as usize;
    let x = tokens.next().expect("Missing x") as usize;

    // Parse nums_size and nums array
    let nums_size = tokens.next().expect("Missing nums_size") as usize;
    let nums: Vec<i32> = tokens.by_ref().take(nums_size).collect();

    // Check if we have enough numbers
    if nums.len() != nums_size {
        panic!("Not enough numbers in input");
    }

    // Compute answer
    let answer = find_x_sum(&nums, k, x);

    // Print results with trailing space and newline as per original code
    for num in answer {
        print!("{} ", num);
    }
    println!();
}