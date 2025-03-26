use std::io::{self, BufRead};

/// Merges overlapping or adjacent intervals where the interval length
/// does not exceed `len`. Returns the size of the merged intervals.
///
/// Translated from:
/// int merge(int intervals[][2], int size, int len, int merged[][2])
fn merge(intervals: &[[i32; 2]], size: usize, len: i32, merged_out: &mut Vec<[i32; 2]>) -> usize {
    if size == 0 {
        return 0;
    }

    let mut merged_size = 0usize;
    let mut current_start = intervals[0][0];
    let mut current_end = intervals[0][1];

    for i in 1..size {
        let start = intervals[i][0];
        let end = intervals[i][1];
        // If intervals overlap and the combined length does not exceed `len`, merge them
        if start <= current_end && (start - current_start + 1) <= len {
            current_end = current_end.max(end);
        } else {
            merged_out.push([current_start, current_end]);
            merged_size += 1;
            current_start = start;
            current_end = end;
        }
    }

    merged_out.push([current_start, current_end]);
    merged_size + 1
}

/// Checks if it is possible to have at most `op` merged intervals
/// of fully '0' or fully '1' substrings of length `mid`.
///
/// Translated from:
/// bool isPoss(char *s, int n, int op, int mid)
fn is_poss(s: &str, n: i32, op: i32, mid: i32) -> bool {
    let mut i_usize: usize = 0;
    let mut j_usize: usize = 0;
    let mut zero_count: i32 = 0;
    let mut one_count: i32 = 0;

    let mut intervals: Vec<[i32; 2]> = Vec::new();

    let n_usize = n as usize;
    while j_usize < n_usize {
        if s.as_bytes()[j_usize] == b'0' {
            zero_count += 1;
        } else {
            one_count += 1;
        }

        // Shrink the window if its size exceeds `mid`
        while (j_usize as i32 - i_usize as i32 + 1) > mid {
            if s.as_bytes()[i_usize] == b'0' {
                zero_count -= 1;
            } else {
                one_count -= 1;
            }
            i_usize += 1;
        }

        // If the window size is exactly `mid`, check if it's all '0' or all '1'
        if (j_usize as i32 - i_usize as i32 + 1) == mid {
            if zero_count == 0 || one_count == 0 {
                intervals.push([i_usize as i32, j_usize as i32]);
            }
        }

        j_usize += 1;
    }

    // Merge intervals
    let mut merged: Vec<[i32; 2]> = Vec::new();
    let merged_size = merge(&intervals, intervals.len(), mid, &mut merged);

    merged_size as i32 <= op
}

/// Counts the number of positions to change so that:
///  - Even indices match `even`
///  - Odd indices match `odd`
///
/// Translated from:
/// int getMini(char *s, int n, char even, char odd)
fn get_mini(s: &str, n: i32, even: u8, odd: u8) -> i32 {
    let mut op: i32 = 0;
    for i in 0..(n as usize) {
        if i % 2 == 0 {
            if s.as_bytes()[i] != even {
                op += 1;
            }
        } else {
            if s.as_bytes()[i] != odd {
                op += 1;
            }
        }
    }
    op
}

/// Determines the minimum length of a fully '0' or fully '1' substring
/// that cannot be removed with at most `num_ops` merges/replacements.
///
/// Translated from:
/// int minLength(char *s, int numOps)
fn min_length(s: &str, num_ops: i32) -> i32 {
    let n = s.len() as i32;
    // Binary substrings shorter than 3 are handled specially in the original code
    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    // Compute minimal replacements if we force positions to alternate '0'/'1' or '1'/'0'
    let mut op = i32::MAX;
    let op_even_odd = get_mini(s, n, b'0', b'1');  // even=0, odd=1
    let op_odd_even = get_mini(s, n, b'1', b'0');  // even=1, odd=0
    op = op.min(op_even_odd);
    op = op.min(op_odd_even);

    // If we can achieve a single-character answer
    if op <= num_ops {
        return 1;
    }

    // Binary search for the minimum length
    while start <= end {
        let mid = start + (end - start) / 2;
        if is_poss(s, n, num_ops, mid) {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }

    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the binary string from the first line
    let s = lines.next().ok_or("Missing string input")??;
    // Read the number of operations from the second line
    let num_ops_str = lines.next().ok_or("Missing numOps input")??;
    let num_ops: i32 = num_ops_str.trim().parse()?;

    // Compute the minimum length
    let result = min_length(&s.trim(), num_ops);

    // Output the result
    println!("{}", result);

    Ok(())
}