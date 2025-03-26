use std::io::{self, BufRead};

// Define the Element struct with value and count.
#[derive(Debug, Clone, Copy)]
struct Element {
    value: i32,
    count: i32,
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i32> {
    // Number of sliding windows
    let num_results = nums.len() as isize - k as isize + 1;
    let mut answer = Vec::with_capacity(num_results as usize);

    // Process every sliding window
    for i in 0..(nums.len() - k + 1) {
        // frequency array of size 51 initialised to zero
        let mut hash = [0; 51];
        for j in 0..k {
            let num = nums[i + j] as usize;
            hash[num] += 1;
        }

        // Build elements vector containing elements with count > 0
        let mut elements: Vec<Element> = Vec::new();
        for (value, &cnt) in hash.iter().enumerate() {
            if cnt > 0 {
                elements.push(Element {
                    value: value as i32,
                    count: cnt,
                });
            }
        }

        // Sort the elements:
        // Primary: count descending, Secondary: value descending
        elements.sort_unstable_by(|a, b| {
            if a.count == b.count {
                b.value.cmp(&a.value)
            } else {
                b.count.cmp(&a.count)
            }
        });

        // Sum over the top x elements (or all if less than x)
        let elements_to_sum = elements.len().min(x);
        let sum: i32 = elements
            .iter()
            .take(elements_to_sum)
            .map(|element| element.value * element.count)
            .sum();

        answer.push(sum);
    }

    answer
}

fn main() {
    // Read entire input from stdin using BufReader for efficient reading.
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|line| line.unwrap());

    // Collect all tokens from the input in a vector.
    let mut tokens = Vec::new();
    for line in iterator {
        // Split by whitespace and extend tokens
        tokens.extend(line.split_whitespace().map(String::from));
    }

    // Create an iterator to parse tokens one by one
    let mut token_iter = tokens.into_iter();

    // Read k and x from input
    let k: usize = token_iter
        .next()
        .expect("Expected k")
        .parse()
        .expect("k should be an integer");
    let x: usize = token_iter
        .next()
        .expect("Expected x")
        .parse()
        .expect("x should be an integer");

    // Read numsSize
    let nums_size: usize = token_iter
        .next()
        .expect("Expected numsSize")
        .parse()
        .expect("numsSize should be an integer");

    // Read the nums array.
    let mut nums = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        let num = token_iter
            .next()
            .expect("Expected a number in nums")
            .parse()
            .expect("Each number in nums should be an integer");
        nums.push(num);
    }
    
    // Call find_x_sum and obtain the computed result vector.
    let answer = find_x_sum(&nums, k, x);
    
    // Output the answer in the exact same format: each number followed by a space, then newline.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for num in answer {
        // Write each number followed by a space
        write!(handle, "{} ", num).expect("Failed to write to stdout");
    }
    // Write a newline character at the end
    writeln!(handle).expect("Failed to write newline to stdout");
}