use std::io::{self, BufRead};
use std::cmp;
use std::collections::HashMap;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    const INT_MAX: i32 = i32::MAX;
    let mut index = vec![INT_MAX; 100005];

    // Populate the index array with the minimum index of each element
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = cmp::min(index[element as usize], i as i32);
    }

    let mut ans = vec![INT_MAX; groups.len()];

    for (i, &group) in groups.iter().enumerate() {
        for j in 1..=((group as f64).sqrt() as i32) {
            if group % j == 0 {
                let divisor1 = j;
                let divisor2 = group / j;

                if index[divisor1 as usize] != INT_MAX || index[divisor2 as usize] != INT_MAX {
                    ans[i] = cmp::min(
                        ans[i],
                        cmp::min(index[divisor1 as usize], index[divisor2 as usize]),
                    );
                }
            }
        }
        if ans[i] == INT_MAX {
            ans[i] = -1;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let n: usize = first_line_iter.next().unwrap().parse().unwrap();
    let m: usize = first_line_iter.next().unwrap().parse().unwrap();

    // Read groups
    let second_line = lines.next().unwrap().unwrap();
    let groups: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Read elements
    let third_line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = third_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Compute the result
    let ans = assign_elements(&groups, &elements);

    // Print the result
    for (i, &value) in ans.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", value);
    }
    println!();
}