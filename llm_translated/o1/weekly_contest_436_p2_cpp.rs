// Translation of the C++ solution for LeetCode Weekly Contest 436 Problem 2 into Rust.
// This program preserves the exact same input/output format and algorithmic logic as the original C++ code.

use std::io::{self, Read};

// A struct to mirror the "Solution" class in C++
struct Solution;

impl Solution {
    // This function assigns each group element an index based on the multiples in "elements"
    fn assign_elements(&self, mut groups: Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
        // Find the maximum value among the elements
        let &mx = elements.iter().max().expect("Elements array is empty");
        let mx_usize = mx as usize;

        // Create a "target" array of size (mx + 1), initialized to -1
        let mut target = vec![-1; mx_usize + 1];

        // For each element, mark all its multiples in the "target" array if they are not yet set
        for (i, &x) in elements.iter().enumerate() {
            // If x is out of range or already assigned, skip
            if x <= 0 || x as usize > mx_usize || target[x as usize] >= 0 {
                continue;
            }
            let step = x as usize;
            let mut y = step;
            while y <= mx_usize {
                if target[y] < 0 {
                    target[y] = i as i32;
                }
                y += step;
            }
        }

        // Replace each value in groups with its corresponding index from "target"
        for g in groups.iter_mut() {
            if *g >= 0 && (*g as usize) <= mx_usize {
                *g = target[*g as usize];
            } else {
                // If the group value is out of range, mimic the C++ behavior
                // (the original C++ code would go out of bounds if *g > mx,
                // but here we set it safely to -1)
                *g = -1;
            }
        }

        groups
    }
}

fn main() {
    // Read the entire standard input into a string
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    let mut tokens = input.split_whitespace();

    // Parse n and m
    let n = tokens
        .next()
        .expect("Missing input for n")
        .parse::<usize>()
        .expect("Failed to parse n");
    let m = tokens
        .next()
        .expect("Missing input for m")
        .parse::<usize>()
        .expect("Failed to parse m");

    // Read n integers into "groups"
    let mut groups = Vec::with_capacity(n);
    for _ in 0..n {
        let val = tokens
            .next()
            .expect("Not enough integers for groups")
            .parse::<i32>()
            .expect("Failed to parse integer for groups");
        groups.push(val);
    }

    // Read m integers into "elements"
    let mut elements = Vec::with_capacity(m);
    for _ in 0..m {
        let val = tokens
            .next()
            .expect("Not enough integers for elements")
            .parse::<i32>()
            .expect("Failed to parse integer for elements");
        elements.push(val);
    }

    // Create a Solution and call the method
    let solution = Solution;
    let result = solution.assign_elements(groups, &elements);

    // Print the resulting assigned indices, separated by spaces,
    // followed by a single newline (just like the C++ code).
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
}