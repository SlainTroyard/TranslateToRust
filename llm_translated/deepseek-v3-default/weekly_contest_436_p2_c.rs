use std::cmp::min;
use std::io::{self, Write};
use std::collections::HashMap;

fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let mut index = HashMap::new();
    for (i, &element) in elements.iter().enumerate() {
        index.entry(element).or_insert(i32::MAX as usize).min(i);
    }

    let mut ans = Vec::with_capacity(groups.len());
    for &group in groups {
        let mut min_index = i32::MAX;
        for j in 1..=(group as f64).sqrt() as i32 {
            if group % j == 0 {
                if let Some(&idx) = index.get(&j) {
                    min_index = min(min_index, idx as i32);
                }
                if let Some(&idx) = index.get(&(group / j)) {
                    min_index = min(min_index, idx as i32);
                }
            }
        }
        if min_index == i32::MAX {
            ans.push(-1);
        } else {
            ans.push(min_index);
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut groups = Vec::with_capacity(n);
    let mut elements = Vec::with_capacity(m);

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for num in input.split_whitespace() {
        groups.push(num.parse().unwrap());
    }

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    for num in input.split_whitespace() {
        elements.push(num.parse().unwrap());
    }

    let ans = assign_elements(&groups, &elements);
    for &val in &ans {
        print!("{} ", val);
    }
    println!();
}