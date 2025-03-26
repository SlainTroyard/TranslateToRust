use std::collections::HashMap;
use std::io;

const VOWEL_MASK: i32 = 1065233;

fn f(word: &str, k: usize) -> i64 {
    let mut ans: i64 = 0;
    let mut cnt1: HashMap<char, i32> = HashMap::new();
    let mut size1: i32 = 0; // Number of vowel types
    let mut cnt2: i32 = 0;
    let mut left: usize = 0;
    let word_chars: Vec<char> = word.chars().collect();

    for &b in &word_chars {
        let b_idx = (b as u8 - b'a') as i32;
        if (VOWEL_MASK >> b_idx) & 1 != 0 {
            *cnt1.entry(b).or_insert(0) += 1;
            if cnt1[&b] == 1 {
                size1 += 1;
            }
        } else {
            cnt2 += 1;
        }
        while size1 == 5 && cnt2 >= k as i32 {
            let out = word_chars[left] as u8 - b'a';
            if (VOWEL_MASK >> out) & 1 != 0 {
                *cnt1.get_mut(&word_chars[left]).unwrap() -= 1;
                if cnt1[&word_chars[left]] == 0 {
                    size1 -= 1;
                }
            } else {
                cnt2 -= 1;
            }
            left += 1;
        }
        ans += left as i64;
    }
    ans
}

fn count_of_substrings(word: String, k: i32) -> i64 {
    f(&word, k as usize) - f(&word, (k + 1) as usize)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word_size: usize = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word = input.trim().to_string();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: i32 = input.trim().parse().expect("Invalid input");

    let result = count_of_substrings(word, k);
    println!("{}", result);
}