// Translated from C to Rust for LeetCode Weekly Contest 438 Problem 1
// ------------------------------------------------
// The original C code reads a single token from stdin,
// transforms it, then prints 1 or 0 according to the logic.
// We replicate that EXACT behavior here in Rust.

use std::io;

/// Checks whether, after repeatedly "melting" adjacent digits in the array,
/// the first two final digits match. This replicates the same logic from C.
fn has_same_digits(input: &str) -> bool {
    // In C, we had a fixed buffer of size 100. However, the
    // algorithm requires potentially accessing arr[len] when len == 100.
    // So we use 101 here to avoid out-of-bounds issues.
    let mut arr = [0_u8; 101];

    // Convert the input string to bytes. (C code: strlen(s))
    let mut len = input.len();
    // Cap length at 100 to mimic the C buffer limit "char s[100]".
    if len > 100 {
        len = 100;
    }

    // C code: while (i < len) s[i] &= 0x0f;
    // This extracts the numeric value of '0'..'9' by masking the low nibble.
    let bytes = input.as_bytes();
    for i in 0..len {
        arr[i] = bytes[i] & 0x0f; // same as s[i] &= 0x0f in C
    }

    // C code: while (--len > 1) {
    //            for (i = 0; i < len; ++i)
    //                s[i] = (s[i] + s[i + 1]) % 10;
    //         }
    while {
        len -= 1; // pre-decrement
        len > 1
    } {
        for i in 0..len {
            arr[i] = (arr[i] + arr[i + 1]) % 10;
        }
    }

    // C code: return s[0] == s[1];
    // Note: if original input has fewer than 2 digits,
    // the C code may read out-of-bounds. We do the same comparison here,
    // but Rust will still check arr[0] and arr[1] in a valid buffer.
    arr[0] == arr[1]
}

fn main() {
    // C code equivalent:
    // char s[100];
    // scanf("%s", s);
    // printf("%d\n", hasSameDigits(s));

    let mut input = String::new();
    // Read one line from stdin (the first token up to whitespace is used)
    let _ = io::stdin().read_line(&mut input);

    // If there's a token, process it. Otherwise, produce 0 (like the C code
    // which wouldn't initialize s if scanf failed).
    if let Some(token) = input.split_whitespace().next() {
        let result = has_same_digits(token);
        println!("{}", if result { 1 } else { 0 });
    } else {
        // No token read => mimic undefined C behavior by just printing 0.
        println!("0");
    }
}