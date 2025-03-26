use std::io;

fn kth_character(k: usize) -> char {
    // Create a vector with initial values 0
    let mut l = vec![0; k + 1];
    let mut a = 0;

    loop {
        // Iterate over a range [0, 2^a)
        for i in 0..(1 << a) {
            // Calculate j as 2^a
            let j = 1 << a;

            // Update l[i + j] based on l[i]
            l[i + j] = l[i] + 1;

            // If the index is >= k - 1, return the result
            if i + j >= k - 1 {
                return (97 + l[k - 1] % 26) as u8 as char;
            }
        }
        a += 1;
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse input as an integer
    let k: usize = input.trim().parse().expect("Input must be a positive integer");

    // Call the function and print the result
    println!("{}", kth_character(k));
}