use std::io::{self, BufRead, Write};
use std::cmp::Ordering;

// Helper function: compute bit-length of a positive number.
// For a positive number x, bit_length(x) = floor(log2(x)) + 1.
// (For x == 0, we return 1 as its "bit-length" for our purpose.)
fn bit_length(x: u32) -> u32 {
    if x == 0 {
        1
    } else {
        32 - x.leading_zeros()
    }
}

fn max_good_number(mut nums: Vec<u32>) -> u64 {
    // Sort the numbers using the custom comparator.
    // For any two numbers a and b, we compare the values:
    //   left = (a << bit_length(b)) | b
    //   right = (b << bit_length(a)) | a
    //
    // We want a to come before b if left > right.
    // That is, sort in descending order based on the computed concatenation.
    nums.sort_by(|&a, &b| {
        let len_a = bit_length(a);
        let len_b = bit_length(b);
        // Compute concatenated numbers using u64 to avoid overflow.
        let left = ((a as u64) << len_b) | (b as u64);
        let right = ((b as u64) << len_a) | (a as u64);
        // If left > right then a should come before b
        if left > right {
            Ordering::Less
        } else if left < right {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    // Accumulate the final answer by concatenating the binary representations.
    let mut ans: u64 = 0;
    for x in nums {
        let len = bit_length(x);
        ans = (ans << len) | (x as u64);
    }
    ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a buffered reader for standard input.
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();

    // Read entire input.
    reader.read_to_string(&mut input)?;

    // Split the input by whitespace and parse the numbers.
    let mut iter = input.split_whitespace();

    // Read the number of elements.
    let num_size = iter
        .next()
        .ok_or("Expected number of elements")?
        .parse::<usize>()?;

    // Read the vector of numbers.
    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let num = iter
            .next()
            .ok_or("Not enough numbers provided")?
            .parse::<u32>()?;
        nums.push(num);
    }

    // Compute the max good number.
    let ans = max_good_number(nums);

    // Write the answer to stdout, ensuring exact format.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", ans)?;

    Ok(())
}