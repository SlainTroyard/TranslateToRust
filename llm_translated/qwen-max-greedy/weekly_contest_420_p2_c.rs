use std::io;

fn main() {
    // Read input from stdin
    let mut s = String::new();
    let mut k_str = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut k_str).expect("Failed to read line");

    // Trim the newline character and parse k
    let s = s.trim_end();
    let k: usize = k_str.trim_end().parse().expect("Please type a number!");

    // Call the function and print the result
    println!("{}", number_of_substrings(s, k));
}

/// Calculates the number of substrings where each character appears at least k times.
///
/// # Arguments
///
/// * `s` - A string slice representing the input string.
/// * `k` - The minimum number of times each character must appear in the substring.
///
/// # Returns
///
/// * The number of valid substrings.
fn number_of_substrings(s: &str, k: usize) -> usize {
    let mut hash_arr = [0; 26];
    let (mut left, mut right) = (0, 0);
    let s_l = s.len();
    let mut res = 0;
    
    while left < s_l && right < s_l {
        if hash_arr[(s.as_bytes()[right] - b'a') as usize] + 1 == k {
            res += s_l - right;
            while left <= right {
                if hash_arr[(s.as_bytes()[left] - b'a') as usize] - 1 != k - 1 {
                    res += s_l - right;
                } else {
                    break;
                }
                hash_arr[(s.as_bytes()[left] - b'a') as usize] -= 1;
                left += 1;
            }
            right += 1;
        } else {
            hash_arr[(s.as_bytes()[right] - b'a') as usize] += 1;
            right += 1;
        }
    }

    res
}