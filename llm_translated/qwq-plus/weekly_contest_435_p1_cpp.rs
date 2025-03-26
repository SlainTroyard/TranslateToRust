struct Solution;

impl Solution {
    fn max_difference(s: &str) -> i32 {
        let mut cnt = [0; 26]; // Array to count occurrences of each character
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize; // Convert char to 0-based index
            cnt[idx] += 1;
        }

        let mut max1 = 0; // Maximum odd count
        let mut min0 = i32::MAX; // Minimum non-zero even count

        for c in cnt {
            if c % 2 == 1 {
                // Update max1 if current count is odd and larger than current max
                max1 = max1.max(c);
            } else if c > 0 {
                // Update min0 if current count is even and smaller than current min
                if c < min0 {
                    min0 = c;
                }
            }
        }

        max1 - min0 // Compute the difference as per problem requirements
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line"); // Read input with error handling
    let s = input.trim() // Trim whitespace
        .split_whitespace() // Split into tokens
        .next() // Take the first token (like cin >> in C++)
        .expect("No input found") // Panic if no input (as per problem constraints)
        .to_string(); // Convert to String for processing
    println!("{}", Solution::max_difference(&s)); // Output result as per problem
}