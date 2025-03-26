use std::cmp::min;
use std::io::{self, BufRead};

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = vec![i32::MAX; 100005];
    for (i, &element) in elements.iter().enumerate() {
        index[element as usize] = min(index[element as usize], i as i32);
    }

    let mut ans = vec![i32::MAX; groups.len()];
    for (i, &group) in groups.iter().enumerate() {
        for j in 1..=(group as f64).sqrt() as i32 {
            if group % j == 0 && (index[j as usize] != i32::MAX || index[(group / j) as usize] != i32::MAX) {
                ans[i] = min(ans[i], min(index[(group / j) as usize], index[j as usize]));
            }
        }
        if ans[i] == i32::MAX {
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
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read groups
    let second_line = lines.next().unwrap().unwrap();
    let groups: Vec<i32> = second_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Read elements
    let third_line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Assign elements and get the result
    let ans = assign_elements(&groups, &elements);

    // Print the result
    for &val in &ans {
        print!("{} ", val);
    }
    println!();
}