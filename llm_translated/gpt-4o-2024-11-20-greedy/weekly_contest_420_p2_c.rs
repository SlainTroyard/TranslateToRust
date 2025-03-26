use std::io::{self, Write};

fn number_of_substrings(s: &str, k: usize) -> usize {
    let mut hash_arr = [0; 26]; // Array to count occurrences of each character
    let mut left = 0;
    let mut right = 0;
    let s_chars: Vec<char> = s.chars().collect(); // Convert string to a vector of characters
    let s_len = s_chars.len();
    let mut res = 0;

    while left < s_len && right < s_len {
        // Increment the count of the current character
        if {
            hash_arr[s_chars[right] as usize - 'a' as usize] += 1;
            hash_arr[s_chars[right] as usize - 'a' as usize] == k
        } {
            // If the count of the current character reaches `k`
            res += s_len - right; // Add the number of valid substrings ending at `right`
            while left <= right {
                // Shrink the window from the left
                if {
                    hash_arr[s_chars[left] as usize - 'a' as usize] -= 1;
                    hash_arr[s_chars[left] as usize - 'a' as usize] != k - 1
                } {
                    res += s_len - right; // Add more valid substrings
                } else {
                    left += 1; // Move the left pointer
                    break;
                }
                left += 1; // Move the left pointer
            }
            right += 1; // Move the right pointer
        } else {
            right += 1; // Expand the window by moving the right pointer
        }
    }

    res
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    // Split the input into the string `s` and the integer `k`
    let mut parts = input.split_whitespace();
    let s = parts.next().expect("Expected a string input");
    let k: usize = parts
        .next()
        .expect("Expected an integer input")
        .parse()
        .expect("Failed to parse integer");

    // Call the function and print the result
    let result = number_of_substrings(s, k);
    println!("{}", result);
}