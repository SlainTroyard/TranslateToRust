use std::io::{self, BufRead};
use std::cmp;

// Solution struct encapsulates the algorithm logic
struct Solution;

impl Solution {
    pub fn assign_elements(groups: &mut Vec<i32>, elements: &[i32]) -> Vec<i32> {
        let mx = *elements.iter().max().unwrap(); // Find the maximum element
        let mut target = vec![-1; (mx + 1) as usize]; // Initialize target vector with -1
        
        for (i, &x) in elements.iter().enumerate() {
            if x > mx || target[x as usize] >= 0 {
                continue;
            }
            let mut y = x;
            while y <= mx {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32; // Assign index
                }
                y += x; // Increment by x
            }
        }

        for x in groups.iter_mut() {
            *x = target[*x as usize];
        }
        groups.to_vec()
    }
}

fn main() {
    // Create a buffered reader for efficiently reading input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read the second line containing the groups array
    let groups_line = lines.next().unwrap().unwrap();
    let groups: Vec<i32> = groups_line.split_whitespace()
                                      .map(|s| s.parse().unwrap())
                                      .collect();

    // Read the third line containing the elements array
    let elements_line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = elements_line.split_whitespace()
                                          .map(|s| s.parse().unwrap())
                                          .collect();

    // Create a mutable copy of groups
    let mut groups_mut = groups.clone();

    // Use Solution's method to compute the result
    let solution = Solution;
    let result = solution.assign_elements(&mut groups_mut, &elements);

    // Output the result array as a single line
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}