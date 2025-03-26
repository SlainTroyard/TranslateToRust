use std::io;

fn kth_character(k: usize) -> char {
    // Allocate a vector initialized to zero with size k + 1
    let mut l = vec![0; k + 1];
    let mut a = 0;

    loop {
        let j = 1 << a; // Calculate 2^a
        for i in 0..j {
            // Update the value at index i + j
            l[i + j] = l[i] + 1;

            // If we reach the target index (k - 1), return the corresponding character
            if i + j >= k - 1 {
                return (97 + l[k - 1] % 26) as u8 as char; // ASCII 'a' is 97
            }
        }
        a += 1; // Increment a for the next iteration
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse the input as an integer
    let k: usize = input.trim().parse().expect("Invalid input");

    // Call the function and print the result
    println!("{}", kth_character(k));
}