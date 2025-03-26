use std::io::Read;

const VOWEL_MASK: u32 = 1065233;

pub fn count_of_substrings(word: String, k: i32) -> i64 {
    let mut ans = 0i64;
    let mut cnt_vowel1 = [0; 21];
    let mut cnt_vowel2 = [0; 21];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;
    
    for &c in word.as_bytes() {
        let b = (c - b'a') as usize;
        
        if (VOWEL_MASK >> b) & 1 != 0 {
            // Update vowel counts for both windows
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;
            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            // Update consonant counts for both windows
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }
        
        // Adjust left1 to maintain exactly 5 vowels and consonants >=k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out_char = word.as_bytes()[left1];
            let out_b = (out_char - b'a') as usize;
            if (VOWEL_MASK >> out_b) & 1 != 0 {
                cnt_vowel1[out_b] -= 1;
                if cnt_vowel1[out_b] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }
        
        // Adjust left2 to maintain exactly 5 vowels and consonants >k (to find the left boundary)
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out_char = word.as_bytes()[left2];
            let out_b = (out_char - b'a') as usize;
            if (VOWEL_MASK >> out_b) & 1 != 0 {
                cnt_vowel2[out_b] -= 1;
                if cnt_vowel2[out_b] == 0 {
                    size_vowel2 -= 1;
                }
            } else {
                cnt_consonant2 -= 1;
            }
            left2 += 1;
        }
        
        // The valid substrings are those between left2 and left1
        ans += (left1 as i64 - left2 as i64);
    }
    
    ans
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let _word_size: i32 = tokens.next().unwrap().parse().unwrap();
    let word = tokens.next().unwrap().to_string();
    let k: i32 = tokens.next().unwrap().parse().unwrap();
    
    let ans = count_of_substrings(word, k);
    println!("{}", ans);
}