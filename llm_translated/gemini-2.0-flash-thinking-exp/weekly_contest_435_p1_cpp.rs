use std::io;
use std::cmp::{max, min};

fn max_difference(s: String) -> i32 {
    let mut cnt = [0; 26]; // Array to store counts of each character 'a' to 'z'

    // Count the frequency of each character in the string
    for b in s.chars() {
        if 'a' <= b && b <= 'z' {
            cnt[(b as u8 - b'a') as usize] += 1;
        }
    }

    let mut max1 = 0; // Maximum count of characters with odd frequency
    let mut min0 = i32::MAX; // Minimum count of characters with even frequency (non-zero)

    // Iterate through the character counts
    for c in cnt.iter() {
        if c % 2 != 0 { // If count is odd
            max1 = max(max1, *c);
        } else if *c != 0 { // If count is even and non-zero
            min0 = min(min0, *c);
        }
    }

    // If min0 is still INT_MAX, it means no even count was found (or all even counts were zero, which is excluded by the condition *c != 0).
    // In the original C++ code, if min0 is not updated, it remains INT_MAX.
    // The subtraction max1 - min0 in this case will result in a very small negative number.
    // Let's follow the original logic directly.
    if min0 == i32::MAX {
        return max1 - 0; // If no even counts, treat min0 as 0, to align with potential intended behavior when no even counts are found and avoid underflow. Or just return max1 - min0 as is to strictly follow original logic. Let's go with original logic first and see.
    } else {
        return max1 - min0;
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let result = max_difference(s.to_string());
    println!("{}", result);
}