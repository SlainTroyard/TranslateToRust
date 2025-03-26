use std::io::{self, BufRead};

fn main() {
    // Read the first line to get the number of groups and elements
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // Read the groups
    let mut groups = Vec::with_capacity(n);
    let groups_line = lines.next().unwrap().unwrap();
    for num in groups_line.split_whitespace() {
        groups.push(num.parse::<i32>().unwrap());
    }

    // Read the elements
    let mut elements = Vec::with_capacity(m);
    let elements_line = lines.next().unwrap().unwrap();
    for num in elements_line.split_whitespace() {
        elements.push(num.parse::<i32>().unwrap());
    }

    // Call the function and get the result
    let result = assign_elements(groups, elements);

    // Print the result
    for num in result {
        print!("{} ", num);
    }
    println!();
}

fn assign_elements(mut groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
    // Find the maximum element in the elements vector
    let mx = *elements.iter().max().unwrap();

    // Initialize a target vector with -1
    let mut target = vec![-1; (mx + 1) as usize];

    // Iterate over the elements and populate the target vector
    for (i, &x) in elements.iter().enumerate() {
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

    // Update the groups vector with the corresponding indices from the target vector
    for x in &mut groups {
        *x = target[*x as usize];
    }

    groups
}