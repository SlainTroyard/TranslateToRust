use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // We'll use a buffered reader for input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read first string s. Trim to remove any whitespace/newline.
    let s_line = match lines.next() {
        Some(line) => line?,
        None => {
            eprintln!("No input for s");
            return Ok(());
        }
    };
    let s = s_line.trim().to_string();

    // Read second string t.
    let t_line = match lines.next() {
        Some(line) => line?,
        None => {
            eprintln!("No input for t");
            return Ok(());
        }
    };
    let t = t_line.trim().to_string();

    // Read integer k from the next line (or token)
    let k_line = match lines.next() {
        Some(line) => line?,
        None => {
            eprintln!("No input for k");
            return Ok(());
        }
    };
    // Attempt to parse k as integer.
    let k: usize = match k_line.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid integer for k");
            return Ok(());
        }
    };

    // Check if it is possible to rearrange the strings.
    let possible = is_possible_to_rearrange(&s, &t, k);
    
    // Write output exactly as the original code does.
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    if possible {
        writeln!(out, "true")?;
    } else {
        writeln!(out, "false")?;
    }
    
    Ok(())
}

/// This function implements the logic from the original C code.
/// It partitions strings s and t into k equal length segments, sorts the segments
/// lexicographically, and then compares whether the sorted sequences are identical.
fn is_possible_to_rearrange(s: &str, t: &str, k: usize) -> bool {
    // Ensure that each string length is divisible by k.
    if s.len() % k != 0 || t.len() % k != 0 {
        return false;
    }
    let len = s.len() / k; // segment length

    // Partition the strings into k segments.
    let mut s_chunks: Vec<&str> = s.as_bytes()
        .chunks(len)
        .map(|chunk| {
            // SAFETY: we assume input strings are valid UTF-8 and segments are valid too.
            unsafe { std::str::from_utf8_unchecked(chunk) }
        })
        .collect();

    let mut t_chunks: Vec<&str> = t.as_bytes()
        .chunks(len)
        .map(|chunk| {
            unsafe { std::str::from_utf8_unchecked(chunk) }
        })
        .collect();

    // Sort the segments lexicographically using Rust's sort (which is analogous to qsort in C).
    s_chunks.sort();
    t_chunks.sort();

    // Compare the sorted segments.
    s_chunks == t_chunks
}