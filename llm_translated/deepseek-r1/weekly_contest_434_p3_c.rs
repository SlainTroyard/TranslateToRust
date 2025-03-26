use std::io::{self, BufRead};

fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0; // Tracks the count of elements equal to k so far
    let mut f1 = [0; 51]; // Stores maximum frequency for each number (0-50)
    let mut max_f1 = 0; // Tracks the maximum value in f1
    let mut f2 = 0; // Tracks a specific combination of previous max and current k count

    for &x in nums {
        let x_idx = x as usize; // Safe under problem constraints (0 <= x <= 50)

        // Update f2 using previous max and current k status
        f2 = f2.max(max_f1) + (x == k) as i32;

        // Update frequency for current number, considering previous k count
        f1[x_idx] = f1[x_idx].max(f0) + 1;

        // Maintain maximum frequency seen so far
        if f1[x_idx] > max_f1 {
            max_f1 = f1[x_idx];
        }

        // Update k count if current element matches k
        if x == k {
            f0 += 1;
        }
    }

    // Final result is the best of the two strategies
    max_f1.max(f2)
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin.lock().split_whitespace();

    // Read n and k from input
    let n = match tokens.next().and_then(|t| t.ok()).and_then(|s| s.parse::<usize>().ok()) {
        Some(num) => num,
        None => {
            eprintln!("Error reading input for n and k");
            return;
        }
    };

    let k = match tokens.next().and_then(|t| t.ok()).and_then(|s| s.parse::<i32>().ok()) {
        Some(num) => num,
        None => {
            eprintln!("Error reading input for n and k");
            return;
        }
    };

    // Read the array elements
    let mut nums = Vec::with_capacity(n);
    for i in 0..n {
        let num = match tokens.next().and_then(|t| t.ok()).and_then(|s| s.parse::<i32>().ok()) {
            Some(num) => num,
            None => {
                eprintln!("Error reading input for nums[{}]", i);
                return;
            }
        };
        nums.push(num);
    }

    // Calculate and print the result
    println!("{}", max_frequency(&nums, k));
}