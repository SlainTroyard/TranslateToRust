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
    let first_line = lines.next().unwrap().expect("Failed to read the first line");
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = iter.next().unwrap().parse().expect("Failed to parse m");

    // Parse the second line to get the groups
    let second_line = lines.next().unwrap().expect("Failed to read the second line");
    let groups: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse group element"))
        .collect();

    // Parse the third line to get the elements
    let third_line = lines.next().unwrap().expect("Failed to read the third line");
    let elements: Vec<i32> = third_line
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse element"))
        .collect();

    // Create a solution instance and call the function
    let mut result = groups.clone();
    assign_elements(&mut result, &elements);

    // Write the result to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for (i, &x) in result.iter().enumerate() {
        write!(handle, "{}", x).expect("Failed to write to stdout");
        if i < result.len() - 1 {
            write!(handle, " ").expect("Failed to write space to stdout");
        }
    }
    writeln!(handle).expect("Failed to write newline to stdout");
}