use std::io::{self, BufRead, Write};

fn assign_elements(groups: &mut Vec<i32>, elements: &Vec<i32>) -> Vec<i32> {
    let mx = *elements.iter().max().unwrap_or(&0);
    let mut target = vec![-1; (mx + 1) as usize];
    
    for (i, &x) in elements.iter().enumerate() {
        if x > mx || target[x as usize] >= 0 {
            continue;
        }
        for y in (x..=mx).step_by(x as usize) {
            if target[y as usize] < 0 {
                target[y as usize] = i as i32;
            }
        }
    }

    for x in groups.iter_mut() {
        *x = target[*x as usize];
    }
    groups.to_vec()
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line to get n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Parse the second line to get groups
    let groups_line = lines.next().unwrap().unwrap();
    let mut groups: Vec<i32> = groups_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Parse the third line to get elements
    let elements_line = lines.next().unwrap().unwrap();
    let elements: Vec<i32> = elements_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Process the data
    let result = assign_elements(&mut groups, &elements);

    // Write the result to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, &x) in result.iter().enumerate() {
        write!(handle, "{}", x).unwrap();
        if i < result.len() - 1 {
            write!(handle, " ").unwrap();
        }
    }
    writeln!(handle).unwrap();
}