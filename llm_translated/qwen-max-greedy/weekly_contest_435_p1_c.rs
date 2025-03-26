use std::io;

/// Function to calculate the maximum difference between the frequency of any odd-occurring
/// character and the minimum frequency of any even-occurring character.
fn max_difference(s: &str) -> i32 {
    let mut cnt = [0; 26];
    for c in s.chars() {
        if let Some(index) = c.to_lowercase().next() {
            cnt[index as usize - 'a' as usize] += 1;
        }
    }

    let mut c1 = 0;
    let mut c2 = 100;
    for &freq in &cnt {
        if freq & 1 == 1 {
            c1 = c1.max(freq);
        } else if freq != 0 {
            c2 = c2.min(freq);
        }
    }

    c1 - c2
}

fn main() {
    // Read a single line of input from stdin
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the newline character and whitespace
    let input = input.trim();

    // Calculate the result and print it to stdout
    let result = max_difference(input);
    println!("{}", result);
}