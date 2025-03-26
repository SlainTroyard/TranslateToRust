use std::cmp;
use std::collections::BTreeSet;
use std::io::{self, BufRead, Write};

/// Computes the maximum number of distinct elements that can be chosen
/// such that for each element a[i], we choose an integer x satisfying:
/// x is at least prev + 1 and no less than a[i] - diff, and x is no greater than a[i] + diff.
/// This function mirrors the logic given in the C++ solution.
fn max_distinct_elements(arr: &mut Vec<i32>, diff: i32) -> usize {
    // sort the array
    arr.sort();
    let mut prev = i32::MIN;
    let mut chosen = BTreeSet::new();
    
    // iterate over the sorted array
    for &x in arr.iter() {
        // Calculate the candidate x value
        let candidate = cmp::max(prev + 1, x - diff);
        // Check if the candidate is within the allowed range [x - diff, x + diff]
        if candidate <= x + diff {
            chosen.insert(candidate);
            prev = candidate;
        }
    }
    
    chosen.len()
}

fn main() -> io::Result<()> {
    // Use standard input (stdin) and lock it for efficient reading
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input_line = String::new();

    // Read the first line which should contain n, the size of the array.
    // Alternatively, since input might be space separated, we use read_line and split_whitespace.
    reader.read_line(&mut input_line)?;
    // Trim and parse the first input integer (n), the size of the array
    let n: usize = input_line
        .trim()
        .split_whitespace()
        .next()
        .expect("Expected n")
        .parse()
        .expect("Failed to parse n");

    // Clear the buffer and read the next line for diff.
    input_line.clear();
    reader.read_line(&mut input_line)?;
    let diff: i32 = input_line
        .trim()
        .split_whitespace()
        .next()
        .expect("Expected diff")
        .parse()
        .expect("Failed to parse diff");

    // Read array elements.
    // We need to collect n integers. In the C++ code, they are read one by one.
    // Here we will continue reading lines until we have n integers.
    let mut arr = Vec::with_capacity(n);
    while arr.len() < n {
        input_line.clear();
        // Read the next line from stdin
        let bytes_read = reader.read_line(&mut input_line)?;
        if bytes_read == 0 {
            break; // break if EOF is reached unexpectedly
        }
        // For each token in the line, try parsing it as an integer
        for token in input_line.trim().split_whitespace() {
            if arr.len() < n {
                let value: i32 = token.parse().expect("Failed to parse array element");
                arr.push(value);
            } else {
                break;
            }
        }
    }

    // Compute the result using the max_distinct_elements function
    let result = max_distinct_elements(&mut arr, diff);

    // Write the result to stdout
    // Use stdout lock for efficient writing
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    writeln!(writer, "{}", result)?;
    
    Ok(())
}