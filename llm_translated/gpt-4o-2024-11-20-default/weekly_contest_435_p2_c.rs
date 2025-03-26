use std::io;

// Function to calculate absolute value
fn abs_val(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

// Function to return the minimum of two numbers
fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

// Function to return the maximum of two numbers
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// Main distance calculation function
fn max_distance(s: &str, k: i32) -> i32 {
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;

    for (i, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => continue, // Ignore invalid characters
        }

        // Calculate the maximum distance based on the current position
        let current_max = min(abs_val(x) + abs_val(y) + k * 2, (i + 1) as i32);
        ans = max(ans, current_max);
    }

    ans
}

fn main() {
    // Create buffered standard input reader
    let mut input = String::new();

    // Read the input string and integer k
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let inputs: Vec<&str> = input.trim().split_whitespace().collect();

    if inputs.len() != 2 {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    let s = inputs[0]; // The movement string
    let k: i32 = inputs[1].parse().expect("Failed to parse k as integer");

    // Invoke the function and compute the result
    let result = max_distance(s, k);

    // Output the result to stdout
    println!("{}", result);
}