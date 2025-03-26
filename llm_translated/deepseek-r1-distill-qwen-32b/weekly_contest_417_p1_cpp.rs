use std::io;

/// This function calculates the kth character in the sequence where each character
/// is determined by the number of set bits in (k - 1). The character is derived by
/// adding the count of set bits to 'a'.
fn kth_character(k: u64) -> char {
    let count = (k - 1).count_ones();
    let a = b'a' as u8;
    (a + count as u8) as char
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let k: u64 = input.trim().parse().expect("Please enter a valid number");
    println!("{}", kth_character(k));
}