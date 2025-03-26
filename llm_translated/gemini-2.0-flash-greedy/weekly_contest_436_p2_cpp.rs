use std::io;
use std::io::prelude::*;

struct Solution {}

impl Solution {
    pub fn assign_elements(groups: &mut Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
        let mx = *elements.iter().max().unwrap_or(&0);
        let mut target = vec![-1; (mx + 1) as usize];

        for i in 0..elements.len() {
            let x = elements[i];
            if x > mx || target[x as usize] >= 0 {
                continue;
            }
            let mut y = x;
            while y <= mx {
                if target[y as usize] < 0 {
                    target[y as usize] = i as i32;
                }
                y += x;
            }
        }

        for x in groups.iter_mut() {
            *x = target[*x as usize];
        }
        groups.clone()
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let groups: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let elements: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut groups_mut = groups.clone();
    let solution = Solution {};
    let result = solution.assign_elements(&mut groups_mut, &elements);

    for i in 0..n {
        print!("{} ", result[i]);
    }
    println!();
}