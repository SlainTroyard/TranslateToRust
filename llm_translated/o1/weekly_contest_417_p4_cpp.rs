// Translated from C++ to Rust for LeetCode Weekly Contest 417 Problem 4
// Original C++ code commented for reference:
// 
// #include <iostream>
// #include <string>
// #include <vector>
// using namespace std;
//
// class Solution {
// public:
//     char kthCharacter(long long k, vector<int>& operations) {
//         k--;
//         int inc = 0;
//         for (int i = __lg(k); i >= 0; i--) {
//             if (k >> i & 1) {
//                 inc += operations[i];
//             }
//         }
//         return 'a' + inc % 26;
//     }
// };
//
// int main() {
//     long long k;
//     int operationSize;
//     cin >> k >> operationSize;
//     vector<int> operations(operationSize);
//     for (int i = 0; i < operationSize; i++) {
//         cin >> operations[i];
//     }
//     Solution s;
//     cout << s.kthCharacter(k, operations) << endl;
//     return 0;
// }

use std::io::{self, BufRead};

struct Solution;

impl Solution {
    // This method replicates the logic of the original C++ kthCharacter function
    fn kth_character(&self, k: i64, operations: &Vec<i32>) -> char {
        // Decrement k first (k--)
        let mut k = k - 1;
        // This accumulates the total from operations
        let mut inc = 0i32;

        // Equivalent to C++ __lg(k). If k=0, set highest_bit to -1 to skip loop.
        let highest_bit = if k == 0 {
            -1
        } else {
            63 - k.leading_zeros() as i32
        };

        // For each bit from highest to lowest
        for i in (0..=highest_bit).rev() {
            // Check if the i-th bit is set
            if (k >> i) & 1 == 1 {
                // Add the corresponding operation
                inc += operations[i as usize];
            }
        }

        // Compute the character, replicating C++ 'a' + inc % 26 behavior exactly
        let offset = inc % 26;                 // May be negative if inc is negative
        let code = 'a' as i32 + offset;        // ASCII math
        (code as u8) as char                  // Cast to u8 then to char (mimics C++ behavior)
    }
}

fn main() -> io::Result<()> {
    // Read the first line containing k and operationSize
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    let mut parts = input_line.split_whitespace();

    // Parse k (long long in C++) and operationSize (int in C++)
    let k: i64 = parts
        .next()
        .expect("Expected a value for k")
        .parse()
        .expect("Failed to parse k");
    let operation_size: usize = parts
        .next()
        .expect("Expected a value for operationSize")
        .parse()
        .expect("Failed to parse operationSize");

    // Read all operations (vector<int> in C++). The original code
    // reads them in a loop, one by one.
    let mut operations = Vec::with_capacity(operation_size);
    let mut count = 0;

    while count < operation_size {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        for num_str in line.split_whitespace() {
            if count < operation_size {
                let val: i32 = num_str
                    .parse()
                    .expect("Failed to parse operation integer");
                operations.push(val);
                count += 1;
            } else {
                break; // We've read all needed operations
            }
        }
    }

    // Create a Solution instance and compute the result
    let solution = Solution;
    let result_char = solution.kth_character(k, &operations);

    // Print the result exactly as the C++ code does (one char + newline)
    println!("{}", result_char);

    Ok(())
}