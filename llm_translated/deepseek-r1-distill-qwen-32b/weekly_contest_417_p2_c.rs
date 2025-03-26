use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    line
}

fn count_substrings(word: &str, k: i32) -> i32 {
    let len = word.len();
    if len < 5 {
        return 0;
    }

    let mut count = 0;
    for i in 0..=len - 5 {
        let mut vowel_counts = [0; 5];
        let mut non_vowel_count = 0;

        for j in i..len {
            let c = word.as_bytes()[j];
            if c == b'a' {
                vowel_counts[0] += 1;
            } else if c == b'e' {
                vowel_counts[1] += 1;
            } else if c == b'i' {
                vowel_counts[2] += 1;
            } else if c == b'o' {
                vowel_counts[3] += 1;
            } else if c == b'u' {
                vowel_counts[4] += 1;
            } else {
                non_vowel_count += 1;
            }

            if vowel_counts[0] > 0 && vowel_counts[1] > 0 && vowel_counts[2] > 0 && vowel_counts[3] > 0 && vowel_counts[4] > 0 && non_vowel_count == k {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let word_size: usize = read_line().trim().parse().expect("Invalid word size");
    let word_line = read_line();
    let word = word_line.split_whitespace().next().unwrap_or("");
    let k: i32 = read_line().trim().parse().expect("Invalid k");

    let result = count_substrings(word, k);
    println!("{}", result);
}