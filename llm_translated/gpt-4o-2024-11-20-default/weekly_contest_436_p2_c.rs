use std::io::{self, BufRead};
use std::cmp;
use std::usize;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    const MAX_VAL: usize = 100_005;
    let mut index = vec![usize::MAX; MAX_VAL];

    // Fill the `index` array with the minimum index for each element in elements
    for (i, &element) in elements.iter().enumerate() {
        let idx = element as usize;
        if idx < MAX_VAL {
            index[idx] = cmp::min(index[idx], i);
        }
    }

    let mut result = vec![-1; groups.len()];

    // Compute the result for each group
    for (i, &group) in groups.iter().enumerate() {
        let group_value = group as usize;
        let mut min_index = usize::MAX;

        // Traverse divisors of `group`
        for j in 1..=((group_value as f64).sqrt().ceil() as usize) {
            if group_value % j == 0 {
                if j < MAX_VAL && index[j] != usize::MAX {
                    min_index = cmp::min(min_index, index[j]);
                }
                let other_divisor = group_value / j;
                if other_divisor < MAX_VAL && index[other_divisor] != usize::MAX {
                    min_index = cmp::min(min_index, index[other_divisor]);
                }
            }
        }

        if min_index != usize::MAX {
            result[i] = min_index as i32;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().collect::<Result<Vec<_>, _>>().unwrap();

    // First line: read n and m
    let mut iter = input[0].split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    // Second line: read `groups`
    let groups: Vec<i32> = input[1]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Third line: read `elements`
    let elements: Vec<i32> = input[2]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Ensure input sizes match expected `n` and `m`
    assert_eq!(groups.len() as i32, n);
    assert_eq!(elements.len() as i32, m);

    // Get the result and print it
    let result = assign_elements(&groups, &elements);

    // Print the result space-separated
    let output = result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", output);
}