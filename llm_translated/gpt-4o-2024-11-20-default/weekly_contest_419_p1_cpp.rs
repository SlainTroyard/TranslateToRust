use std::io::{self, BufRead};
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: usize, x: usize) -> Vec<i32> {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut res = Vec::new();
        let mut l = 0;

        for r in 0..nums.len() {
            // Increment the count of the current number in the window
            *mp.entry(nums[r]).or_insert(0) += 1;

            // Check if the sliding window size equals k
            if r - l + 1 == k {
                // Convert HashMap to a vector for sorting
                let mut vec: Vec<(i32, i32)> = mp.iter().map(|(&num, &cnt)| (num, cnt)).collect();
                // Sort first by count descending, then by number descending
                vec.sort_by(|a, b| {
                    if a.1 == b.1 {
                        b.0.cmp(&a.0) // If counts are equal, sort by number descending
                    } else {
                        b.1.cmp(&a.1) // Otherwise, sort by count descending
                    }
                });

                // Calculate the sum of the top `x` elements in the sorted vector
                let mut sum = 0;
                for (i, &(num, cnt)) in vec.iter().enumerate() {
                    if i >= x {
                        break;
                    }
                    sum += num * cnt;
                }
                res.push(sum);

                // Remove the left-most element from the window
                if let Some(cnt) = mp.get_mut(&nums[l]) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        mp.remove(&nums[l]);
                    }
                }
                l += 1; // Move the left pointer of the sliding window
            }
        }
        
        res
    }
}

fn main() {
    // Read from `stdin`
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Parse k and x
    let first_line = iterator.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let k: usize = first_iter.next().unwrap().parse().unwrap();
    let x: usize = first_iter.next().unwrap().parse().unwrap();

    // Parse the size of nums array
    let second_line = iterator.next().unwrap().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    // Parse the nums array
    let third_line = iterator.next().unwrap().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    // Solve the problem using the `find_x_sum` method
    let solution = Solution;
    let result = solution.find_x_sum(nums, k, x);

    // Print the result
    for (i, &val) in result.iter().enumerate() {
        print!("{}", val);
        if i != result.len() - 1 {
            print!(" ");
        }
    }
    println!();  // Print a newline at the end
}