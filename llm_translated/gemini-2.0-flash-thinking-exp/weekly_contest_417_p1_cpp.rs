fn kth_character(k: i64) -> char {
    let popcount = (k - 1).count_ones();
    ('a' as u8 + popcount as u8) as char
}

fn main() {
    use std::io;

    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read line");
    let k: i64 = k_str.trim().parse().expect("Please enter a valid integer");

    let result = kth_character(k);
    println!("{}", result);
}