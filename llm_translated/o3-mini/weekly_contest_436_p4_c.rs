use std::cmp;
use std::io::{self, BufRead, Write};

/// Returns the minimum element in the slice.
/// This is equivalent to the C function min_element.
fn min_element(arr: &[i32]) -> i32 {
    // Unwrap is safe here because the input array is assumed to be non-empty.
    *arr.iter().min().unwrap()
}

/// Checks if it is possible to achieve at least the given score `low`
/// with the provided points array and `m` moves.
/// This function translates the logic in the C function `check`.
fn check(points: &[i32], m: i64, low: i64) -> bool {
    let n = points.len();
    let mut rem = m; // remaining moves (m)
    let mut pre = 0_i64; // previous k-1 value
    
    for (i, &p) in points.iter().enumerate() {
        // Calculate minimal k needed for current section.
        // Here, (low - 1) / p calculates division in integer arithmetic.
        let mut k = ((low - 1) / (p as i64) + 1) - pre;
        
        // For the last element, if k is already <= 0, then no additional cost is needed.
        if i == n - 1 && k <= 0 {
            break;
        }
        
        // ensure k is at least 1
        k = cmp::max(k, 1);
        
        // deduct the moves cost for this part:
        // The cost is (k * 2 - 1)
        rem -= k * 2 - 1;
        if rem < 0 {
            return false;
        }
        pre = k - 1;
    }
    true
}

/// Computes the maximum achievable score given an array of points,
/// number of points and the operation moves `m`.
/// This function directly translates the C function `maxScore`.
fn max_score(points: &[i32], m: i32) -> i64 {
    // Left bound for binary search.
    let mut left = 0_i64;
    // Compute right bound using min element in points.
    let min_val = min_element(points);
    // Making sure to compute (m+1)/2 with integer division.
    let right = (((m + 1) / 2) as i64) * (min_val as i64) + 1;
    let mut right = right; // mutable bound for binary search

    // Binary search for the maximum possible score.
    while left + 1 < right {
        let mid = left + (right - left) / 2;
        if check(points, m as i64, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}

/// Main function
fn main() -> io::Result<()> {
    // Create a buffered reader for stdin.
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    // Read the entire input into a string.
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    // Split the input by whitespace and create an iterator over tokens.
    let mut tokens = input.split_whitespace();

    // Parse n and m from the input.
    let n: usize = match tokens.next() {
        Some(token) => token.parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "Error reading input for n")
        })?,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Error reading input for n and m",
            ))
        }
    };
    let m: i32 = match tokens.next() {
        Some(token) => token.parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "Error reading input for m")
        })?,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Error reading input for n and m",
            ))
        }
    };

    // Read the points array.
    let mut points = Vec::with_capacity(n);
    for i in 0..n {
        let point = match tokens.next() {
            Some(token) => token.parse::<i32>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Error reading input for points[{}]", i),
                )
            })?,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Error reading input for points[{}]", i),
                ))
            }
        };
        points.push(point);
    }

    // Compute the result using max_score.
    let result = max_score(&points, m);

    // Write the result to stdout.
    // Using stdout().lock() to flush the output.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", result)?;

    Ok(())
}