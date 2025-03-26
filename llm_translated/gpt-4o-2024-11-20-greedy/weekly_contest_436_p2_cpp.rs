use std::io::{self, BufRead};

struct Solution;

impl Solution {
    pub fn assign_elements(mut groups: Vec<usize>, elements: Vec<usize>) -> Vec<isize> {
        // Find the maximum value in `elements`
        let mx = *elements.iter().max().unwrap_or(&0);

        // Initialize the `target` array
        let mut target = vec![-1; mx + 1];
        for (i, &x) in elements.iter().enumerate() {
            if x > mx || target[x] >= 0 {
                // Skip if `x` is out of bounds or already assigned in the `target`
                continue;
            }
            // Assign the index `i` to all multiples of `x`
            for y in (x..=mx).step_by(x) {
                if target[y] < 0 {
                    target[y] = i as isize;
                }
            }
        }

        // Map each value in `groups` to its corresponding value in `target`
        for x in &mut groups {
            *x = target[*x] as usize;
        }

        groups.iter().map(|&x| x as isize).collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    // Read the first line containing n and m
    let first_line = iterator.next().unwrap().unwrap();
    let mut tokens = first_line.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();

    // Read the groups vector
    let groups_line = iterator.next().unwrap().unwrap();
    let groups: Vec<usize> = groups_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read the elements vector
    let elements_line = iterator.next().unwrap().unwrap();
    let elements: Vec<usize> = elements_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Solve the problem using the Solution struct
    let result = Solution::assign_elements(groups, elements);

    // Print the result vector
    let result_str = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result_str);
}