use std::io::{self, BufRead};
use std::vec;

struct Solution {}

impl Solution {
    pub fn assign_elements(groups: &mut Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
        if elements.is_empty() {
            return groups.clone(); // Handle empty elements case, though problem constraints might prevent this
        }
        let mx = *elements.iter().max().unwrap_or(&0); // Find the maximum element in 'elements'
        let mut target: Vec<i32> = vec![-1; (mx + 1) as usize]; // Initialize 'target' vector with -1

        for i in 0..elements.len() {
            let x = elements[i];
            if x > mx || target[x as usize] >= 0 {
                continue; // Skip if x is greater than mx or target[x] is already assigned
            }
            let mut y = x;
            while y <= mx {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32; // Assign the index 'i' to target[y] if it's -1
                }
                y += x; // Move to the next multiple of x
            }
        }

        for x in groups.iter_mut() {
            if (*x as usize) < target.len() { // Check if x is within target bounds
                *x = target[*x as usize]; // Update each element in 'groups' with the corresponding value from 'target'
            } else {
                *x = -1; // Handle cases where group element is out of target range, mirroring potential C++ behavior if target access is out of bounds (though vector access in C++ would usually be checked or cause undefined behavior).  For robustness, and since the original logic doesn't explicitly handle group values larger than mx, we default to -1.
            }
        }
        groups.clone() // Return the modified 'groups' vector
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut nm_iter = first_line.split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = nm_iter.next().unwrap().parse().expect("Failed to parse m");

    let groups_line = lines.next().unwrap().expect("Failed to read line");
    let groups_str: Vec<&str> = groups_line.split_whitespace().collect();
    let mut groups: Vec<i32> = Vec::with_capacity(n);
    for s in groups_str {
        groups.push(s.parse().expect("Failed to parse group element"));
    }

    let elements_line = lines.next().unwrap().expect("Failed to read line");
    let elements_str: Vec<&str> = elements_line.split_whitespace().collect();
    let elements: Vec<i32> = Vec::with_capacity(m);
    for s in elements_str {
        elements.push(s.parse().expect("Failed to parse element"));
    }

    let solution = Solution {};
    let result = solution.assign_elements(&mut groups, &elements);

    for i in 0..n {
        print!("{} ", result[i]);
    }
    println!();
}