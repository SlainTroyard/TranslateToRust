use std::io::{self, BufRead};

// This function implements the algorithm from the original C++ code.
// It takes two vectors: 'groups' and 'elements', and returns a vector with the assigned indices.
fn assign_elements(mut groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32> {
    // Find the maximum element in the 'elements' vector.
    let mx = *elements.iter().max().expect("elements vector should not be empty");

    // Initialize the target vector with size (mx + 1) and fill it with -1.
    // We use mx as an index, so we ensure the vector has length mx+1.
    let mut target = vec![-1; (mx as usize) + 1];

    // Loop through each element in 'elements' along with its index.
    for (i, &x) in elements.iter().enumerate() {
        // Skip if x is greater than mx (should not happen given mx is maximum) 
        // or if we have already assigned a value for target[x].
        if x > mx || target[x as usize] >= 0 {
            continue;
        }
        // For every multiple y of x (from x to mx inclusive):
        // Note: step_by requires the step value to be of type usize.
        for y in (x..=mx).step_by(x as usize) {
            if target[y as usize] < 0 {
                target[y as usize] = i as i32;
            }
        }
    }

    // For each value in groups, assign the corresponding value from target.
    for x in groups.iter_mut() {
        *x = target[*x as usize];
    }
    groups
}

// Main function to handle input and output.
fn main() {
    // Read the entire input from standard input.
    // The input is expected to be: first two integers n and m,
    // followed by n integers for 'groups' and then m integers for 'elements'.
    let stdin = io::stdin();
    let mut input = String::new();
    // Read all input at once.
    stdin
        .lock()
        .read_to_string(&mut input)
        .expect("Failed to read from stdin");

    // Split the input into tokens by whitespace.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut iter = tokens.iter();

    // Parse the number of elements in groups (n) and elements (m).
    let n: usize = iter
        .next()
        .expect("Missing n")
        .parse()
        .expect("Invalid integer for n");
    let m: usize = iter
        .next()
        .expect("Missing m")
        .parse()
        .expect("Invalid integer for m");

    // Parse the n integers for groups.
    let groups: Vec<i32> = (0..n)
        .map(|_| {
            iter.next()
                .expect("Missing value in groups")
                .parse()
                .expect("Invalid integer in groups")
        })
        .collect();

    // Parse the m integers for elements.
    let elements: Vec<i32> = (0..m)
        .map(|_| {
            iter.next()
                .expect("Missing value in elements")
                .parse()
                .expect("Invalid integer in elements")
        })
        .collect();

    // Call the assign_elements function with the parsed input.
    let result = assign_elements(groups, elements);

    // Print the result with each value separated by a space.
    // The output format matches the original C++ code.
    let output: String = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}