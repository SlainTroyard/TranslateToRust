use std::cmp::max;
use std::error::Error;
use std::io::{self, BufRead};

/// Merge overlapping intervals according to the rules from the C code.
///
/// The function takes a slice of intervals (each a tuple (start, end))
/// and the length of the window (len). It merges intervals if:
///   - The current interval starts before or at the end of the previous one,
///   - And the span from the first interval's start to the current start (plus 1)
///     is less than or equal to len.
///
/// Returns a vector of merged intervals.
fn merge(intervals: &[(usize, usize)], len: usize) -> Vec<(usize, usize)> {
    if intervals.is_empty() {
        return Vec::new();
    }

    let mut merged = Vec::with_capacity(intervals.len());
    let (mut current_start, mut current_end) = intervals[0];

    for &(start, end) in intervals.iter().skip(1) {
        if start <= current_end && (start - current_start + 1) <= len {
            current_end = max(current_end, end);
        } else {
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }
    merged.push((current_start, current_end));
    merged
}

/// Checks whether it is possible to cover all windows of length `mid`
/// that are all zeroes or all ones with at most `op` operations.
/// 
/// It does so by using a sliding window technique over the binary string `s`
/// and collecting all windows that contain only one type of digit.
/// Then, overlapping or adjacent intervals are merged as per rules.
///
/// Returns `true` if the number of merged intervals is less than or equal to `op`.
fn is_poss(s: &[u8], n: usize, op: usize, mid: usize) -> bool {
    let mut intervals = Vec::new();
    let mut i = 0;
    let mut j = 0;
    let mut zero = 0;
    let mut one = 0;

    while j < n {
        // Increase count based on the character
        if s[j] == b'0' {
            zero += 1;
        } else {
            one += 1;
        }

        // Shrink the window until its length is <= mid
        while (j - i + 1) > mid {
            if s[i] == b'0' {
                zero -= 1;
            } else {
                one -= 1;
            }
            i += 1;
        }

        // If the window length is exactly mid and the window is all 0's or all 1's,
        // add the interval.
        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals.push((i, j));
            }
        }
        j += 1;
    }

    // Merge intervals as per the merge function.
    let merged_intervals = merge(&intervals, mid);
    merged_intervals.len() <= op
}

/// Counts the number of operations needed to convert the string `s`
/// into an alternating string defined by the characters `even` for even indices
/// and `odd` for odd indices.
fn get_mini(s: &[u8], n: usize, even: u8, odd: u8) -> usize {
    let mut op = 0;
    for i in 0..n {
        if i % 2 == 0 {
            if s[i] != even {
                op += 1;
            }
        } else {
            if s[i] != odd {
                op += 1;
            }
        }
    }
    op
}

/// Computes the minimum possible length of the string after applying
/// a number of operations given by `num_ops`, according to the contest problem.
///
/// If the string can be converted into an alternating string with at most
/// `num_ops` operations, returns 1. Otherwise, uses binary search over
/// the possible lengths (from 3 to n) and finds the smallest length that
/// does not allow the transformation with the given operations.
fn min_length(s: &[u8], num_ops: usize) -> usize {
    let n = s.len();
    // Binary search bounds: start at 3 since lengths 1 and 2 appear trivial,
    // and initialize result as 2.
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    // Check possibility of obtaining an alternating string by flipping characters.
    let op_even_odd = get_mini(s, n, b'0', b'1');
    let op_odd_even = get_mini(s, n, b'1', b'0');
    let op = op_even_odd.min(op_odd_even);

    if op <= num_ops {
        return 1;
    }

    // Binary search for the minimum window length (mid)
    // where it is NOT possible to cover all same-character windows
    // with at most num_ops operations.
    while start <= end {
        let mid = start + (end - start) / 2;
        if is_poss(s, n, num_ops, mid) {
            // If it is possible to cover windows with mid, try a smaller mid.
            end = mid - 1;
        } else {
            // Otherwise, mid is a candidate.
            res = mid;
            start = mid + 1;
        }
    }
    res
}

/// Main function handling I/O and invoking computation.
///
/// Reads a binary string and a number (of operations allowed)
/// from standard input. Uses the exact same input parsing logic as the original C code,
/// and outputs the result to standard output.
fn main() -> Result<(), Box<dyn Error>> {
    // Read entire input from stdin and split by whitespace.
    let stdin = io::stdin();
    let mut tokens = stdin.lock().lines().filter_map(Result::ok).flat_map(|line| {
        line.split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>()
    });

    // The first token is the binary string.
    let s = if let Some(token) = tokens.next() {
        token
    } else {
        return Err("Expected a binary string as input".into());
    };

    // The second token should be the number of allowed operations.
    let num_ops = if let Some(token) = tokens.next() {
        token.parse::<usize>()?
    } else {
        return Err("Expected an integer for number of operations".into());
    };

    // Convert the binary string into a byte slice for easier indexing.
    let s_bytes = s.as_bytes();

    // Compute the minimum length according to the problem logic.
    let result = min_length(s_bytes, num_ops);

    // Output the result exactly as in the C code.
    println!("{}", result);

    Ok(())
}