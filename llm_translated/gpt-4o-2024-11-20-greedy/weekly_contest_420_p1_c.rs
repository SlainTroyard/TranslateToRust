use std::io::{self, Write};
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::slice;

fn string_sequence(target: &str) -> Vec<String> {
    // Calculate the total size of the result array
    let mut return_size = 0;
    for &byte in target.as_bytes() {
        return_size += (byte - b'a' + 1) as usize;
    }

    // Allocate the result vector
    let mut ans = Vec::with_capacity(return_size);

    // Generate the sequence
    for (l, &byte) in target.as_bytes().iter().enumerate() {
        for c in b'a'..=byte {
            let mut s = String::with_capacity(l + 1);
            s.push_str(&target[..l]);
            s.push(c as char);
            ans.push(s);
        }
    }

    ans
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let target = input.trim();

    // Generate the sequence
    let ans = string_sequence(target);

    // Print the result
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for s in ans {
        write!(handle, "{} ", s).expect("Failed to write output");
    }
    writeln!(handle).expect("Failed to write newline");
}