use std::io::{self, BufRead};

fn assign_elements(mut groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
    let mx = *elements.iter().max().expect("elements must not be empty");
    let mut target = vec![-1; (mx + 1) as usize];
    
    for (i, &x) in elements.iter().enumerate() {
        if x > mx {
            continue;
        }
        let xu = x as usize;
        if target[xu] != -1 {
            continue;
        }
        // Mark all multiples of x up to mx
        let mut y = x;
        while y <= mx {
            let yu = y as usize;
            if target[yu] == -1 {
                target[yu] = i as i32;
            }
            y += x;
        }
    }
    
    for x in &mut groups {
        let val = *x;
        *x = if val <= mx && val >= 0 {
            target[val as usize]
        } else {
            -1
        };
    }
    
    groups
}

fn main() {
    let stdin = io::stdin();
    let mut tokens = stdin.lock()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    
    let n = tokens.next().unwrap() as usize;
    let m = tokens.next().unwrap() as usize;
    
    let groups: Vec<i32> = (0..n).map(|_| tokens.next().unwrap()).collect();
    let elements: Vec<i32> = (0..m).map(|_| tokens.next().unwrap()).collect();
    
    let result = assign_elements(groups, elements);
    
    println!(
        "{}",
        result.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}