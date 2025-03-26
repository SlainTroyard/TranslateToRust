use std::io;
use std::io::Read;
use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn assign_elements(groups: &mut Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
        let mut mx = 0;
        for &x in elements {
            mx = max(mx, x);
        }
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
        groups.to_vec()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut first_split = first_line.split_whitespace();
    let n = first_split.next().unwrap().parse::<usize>().unwrap();
    let m = first_split.next().unwrap().parse::<usize>().unwrap();

    let mut groups: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let elements: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let solution = Solution {};
    let result = solution.assign_elements(&mut groups, &elements);

    for i in 0..n {
        print!("{} ", result[i]);
    }
    println!();
}