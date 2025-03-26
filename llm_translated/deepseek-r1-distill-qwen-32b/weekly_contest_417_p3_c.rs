const VOWEL_MASK: u64 = 1065233;

fn count_of_substrings(word: &str, k: i32) -> i64 {
    let bytes = word.as_bytes();
    let length = bytes.len();
    let mut ans = 0_i64;
    
    let mut cnt_vowel1 = [0; 26];
    let mut cnt_vowel2 = [0; 26];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;
    
    for i in 0..length {
        let c = bytes[i];
        let b = (c - b'a') as usize;
        
        if (VOWEL_MASK >> b) & 1 != 0 {
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
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }
        
        // Shrink window1 to maintain cnt_consonant1 >= k
        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = bytes[left1];
            let out_b = (out - b'a') as usize;
            
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
        
        // Shrink window2 to maintain cnt_consonant2 > k
        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = bytes[left2];
            let out_b = (out - b'a') as usize;
            
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
        
        ans += (left1 - left2) as i64;
    }
    
    ans
}

fn main() {
    use std::io;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read word size");
    let word_size: usize = input.trim().parse().expect("Invalid word size");
    
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read word");
    word = word.trim().to_string();
    assert_eq!(word.len(), word_size, "Word length doesn't match word size");
    
    let mut k_str = String::new();
    io::stdin().read_line(&mut k_str).expect("Failed to read k");
    let k: i32 = k_str.trim().parse().expect("Invalid k");
    
    let result = count_of_substrings(&word, k);
    println!("{}", result);
}