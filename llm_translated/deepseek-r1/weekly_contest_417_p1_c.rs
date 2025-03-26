use std::io;

fn kth_character(k: usize) -> char {
    // Allocate a vector initialized with zeros, size k+1 to accommodate up to index k
    let mut l = vec![0; k + 1];
    let mut a = 0;
    loop {
        let j = 1 << a; // Current level's starting index multiplier
        // Iterate through each position in the current level
        for i in 0..(1 << a) {
            let idx = i + j; // Calculate the index in the current level
            l[idx] = l[i] + 1; // Set value based on parent node
            // If we've reached or passed the target index, return the character
            if idx >= k - 1 {
                // Use modulo 26 to wrap around the alphabet and convert to char
                return (b'a' + (l[k - 1] % 26) as u8) as char;
            }
        }
        a += 1; // Move to the next level
    }
}

fn main() {
    // Read input exactly as per original C code's scanf("%d", &k)
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: usize = input.trim().parse().expect("Invalid input: not an integer");
    
    // Compute and print the result
    let result = kth_character(k);
    println!("{}", result);
}