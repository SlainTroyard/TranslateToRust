use std::io;

fn count_of_substrings(word: &str, k: usize) -> i64 {
    const VOWEL_MASK: u32 = 1065233;
    let mut ans: i64 = 0;
    
    // Initialize vowel count arrays and related variables
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0; // Number of distinct vowels in window1 and window2
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;
    
    for (i, &b) in word.as_bytes().iter().enumerate() {
        let b = (b - b'a') as usize;
        
        if (VOWEL_MASK >> b) & 1 == 1 {
            // Update window1
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;
            
            // Update window2
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            // It's a consonant
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }
        
        // Shrink window1 to maintain cnt_consonant1 >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (word.as_bytes()[left1] - b'a') as usize;
            if (VOWEL_MASK >> out) & 1 == 1 {
                if cnt_vowel1[out] == 1 {
                    size_vowel1 -= 1;
                }
                cnt_vowel1[out] -= 1;
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }
        
        // Shrink window2 to maintain cnt_consonant2 > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (word.as_bytes()[left2] - b'a') as usize;
            if (VOWEL_MASK >> out) & 1 == 1 {
                if cnt_vowel2[out] == 1 {
                    size_vowel2 -= 1;
                }
                cnt_vowel2[out] -= 1;
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }
        
        // Add the number of valid substrings ending at the current character
        ans += (left1 - left2) as i64;
    }
    
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let word_size: usize = input.trim().parse().expect("Please type a number!");
    
    let mut word = String::with_capacity(word_size + 1);
    io::stdin().read_line(&mut word).expect("Failed to read line");
    word = word.trim().to_string();
    
    let mut k_input = String::new();
    io::stdin().read_line(&mut k_input).expect("Failed to read line");
    let k: usize = k_input.trim().parse().expect("Please type a number!");
    
    let result = count_of_substrings(&word, k);
    println!("{}", result);
}