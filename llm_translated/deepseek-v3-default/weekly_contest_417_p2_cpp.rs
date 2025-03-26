use std::collections::HashMap;
use std::io;

const VOWEL_MASK: i32 = 1065233;

fn f(word: &str, k: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut cnt1: HashMap<char, i32> = HashMap::new();
    let mut size1: i32 = 0; // Number of vowel types
    let mut cnt2: i32 = 0;
    let mut left: usize = 0;

    for b in word.chars() {
        let b = b as i32 - 'a' as i32;
        if (VOWEL_MASK >> b) & 1 == 1 {
            let count = cnt1.entry(b as char).or_insert(0);
            *count += 1;
            if *count == 1 {
                size1 += 1;
            }
        } else {
            cnt2 += 1;
        }

        while size1 == 5 && cnt2 >= k {
            let out = word.chars().nth(left).unwrap() as i32 - 'a' as i32;
            if (VOWEL_MASK >> out) & 1 == 1 {
                let count = cnt1.get_mut(&(out as char)).unwrap();
                *count -= 1;
                if *count == 0 {
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

fn count_of_substrings(word: &str, k: i32) -> i64 {
    f(word, k) - f(word, k + 1)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word_size: i32 = input.trim().parse().expect("Invalid input");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word = input.trim();

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: i32 = input.trim().parse().expect("Invalid input");

    let result = count_of_substrings(word, k);
    println!("{}", result);
}