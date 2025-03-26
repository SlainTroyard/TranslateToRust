use std::io::{self, Write};

// Function to calculate the number of substrings
fn number_of_substrings(s: &str, k: usize) -> usize {
    let mut hash_arr = [0; 26];
    let mut left = 0;
    let mut right = 0;
    let s_l = s.len();
    let mut res = 0;

    let s_bytes = s.as_bytes(); // Convert string to bytes for easier ASCII index operations

    while left < s_l && right < s_l {
        // Increment the count for the current character in the window
        if {
            hash_arr[(s_bytes[right] - b'a') as usize] += 1;
            hash_arr[(s_bytes[right] - b'a') as usize]
        } == k
        {
            res += s_l - right;
            while left <= right {
                // Decrement the count for the left-most character
                hash_arr[(s_bytes[left] - b'a') as usize] -= 1;
                res += s_l - right;

                left += 1;
                // Break the loop if reducing the left character breaks the constraint
                if hash_arr[(s_bytes[left - 1] - b'a') as usize] == k - 1 {
                    break;
                }
            }
            right += 1;
        } else {
            right += 1;
        }
    }

    res
}

fn main() {
    // Use stdin to read input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into the string and k
    let mut parts = input.trim().split_whitespace();
    let s = parts.next().expect("Missing input string");
    let k: usize = parts
        .next()
        .expect("Missing input k")
        .parse()
        .expect("Failed to parse k as usize");

    // Call the function and print the result
    let result = number_of_substrings(s, k);
    println!("{}", result);
}