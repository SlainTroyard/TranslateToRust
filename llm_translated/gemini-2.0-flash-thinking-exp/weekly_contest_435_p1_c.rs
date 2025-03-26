fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26]; // Initialize count array for 26 lowercase letters
    let mut c1 = 0; // Initialize max odd count
    let mut c2 = 100; // Initialize min even count

    // Count character occurrences
    for char_byte in s.bytes() {
        if char_byte >= b'a' && char_byte <= b'z' {
            cnt[(char_byte - b'a') as usize] += 1;
        }
    }

    // Find max odd count and min even count
    for i in 0..26 {
        if cnt[i] % 2 != 0 { // Check if count is odd
            c1 = c1.max(cnt[i]); // Update max odd count
        } else if cnt[i] != 0 { // Check if count is even and not zero
            c2 = c2.min(cnt[i]); // Update min even count
        }
    }

    c1 - c2 // Return the difference
}

fn main() {
    use std::io;

    let mut s = String::new(); // Initialize an empty String to store input
    io::stdin().read_line(&mut s).expect("Failed to read line"); // Read a line from stdin

    let s = s.trim(); // Remove trailing newline

    let result = max_difference(&s); // Call max_difference function
    println!("{}", result); // Print the result to stdout
}