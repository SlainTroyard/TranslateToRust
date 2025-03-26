use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read the second line containing the groups
    let groups_line = lines.next().unwrap().unwrap();
    let groups: Vec<i32> = groups_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read the third line containing the elements
    let elements_line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = elements_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Call the assign_elements function
    let result = assign_elements(groups, elements);

    // Print the result
    for val in result {
        print!("{} ", val);
    }
    println!();
}

fn assign_elements(mut groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
    // Find the maximum element in the elements vector
    let mx = *elements.iter().max().unwrap();

    // Initialize a target vector with size mx + 1, filled with -1
    let mut target = vec![-1; (mx + 1) as usize];

    // Iterate over the elements and populate the target vector
    for (i, &x) in elements.iter().enumerate() {
        if x > mx || target[x as usize] >= 0 {
            continue;
        }
        for y in (x..=mx).step_by(x as usize) {
            if target[y as usize] < 0 {
                target[y as usize] = i as i32;
            }
        }
    }

    // Update the groups vector with the corresponding values from the target vector
    for x in &mut groups {
        *x = target[*x as usize];
    }

    groups
}