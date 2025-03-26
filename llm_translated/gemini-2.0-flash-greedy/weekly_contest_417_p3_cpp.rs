use std::io;

const VOWEL_MASK: i32 = 1065233;

fn count_of_substrings(word: String, k: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut cnt_vowel1 = [0; 21];
    let mut cnt_vowel2 = [0; 21];
    let mut size_vowel1 = 0;
    let mut size_vowel2 = 0;
    let mut cnt_consonant1 = 0;
    let mut cnt_consonant2 = 0;
    let mut left1 = 0;
    let mut left2 = 0;

    for (right, &b) in word.as_bytes().iter().enumerate() {
        let b = (b - b'a') as usize;

        if (VOWEL_MASK >> b) & 1 != 0 {
            if cnt_vowel1[b] == 0 {
                size_vowel1 += 1;
            }
            cnt_vowel1[b] += 1;

            if cnt_vowel2[b] == 0 {
                size_vowel2 += 1;
            }
            cnt_vowel2[b] += 1;
        } else {
            cnt_consonant1 += 1;
            cnt_consonant2 += 1;
        }

        while size_vowel1 == 5 && cnt_consonant1 >= k {
            let out = (word.as_bytes()[left1] - b'a') as usize;
            if (VOWEL_MASK >> out) & 1 != 0 {
                cnt_vowel1[out] -= 1;
                if cnt_vowel1[out] == 0 {
                    size_vowel1 -= 1;
                }
            } else {
                cnt_consonant1 -= 1;
            }
            left1 += 1;
        }

        while size_vowel2 == 5 && cnt_consonant2 > k {
            let out = (word.as_bytes()[left2] - b'a') as usize;
            if (VOWEL_MASK >> out) & 1 != 0 {
                cnt_vowel2[out] -= 1;
                if cnt_vowel2[out] == 0 {
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

fn main() -> io::Result<()> {
    let mut word_size = String::new();
    io::stdin().read_line(&mut word_size)?;
    let word_size: i32 = word_size.trim().parse().unwrap();

    let mut word = String::new();
    io::stdin().read_line(&mut word)?;
    let word: String = word.trim().to_string();

    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    let k: i32 = k.trim().parse().unwrap();

    let ans = count_of_substrings(word, k);
    println!("{}", ans);

    Ok(())
}