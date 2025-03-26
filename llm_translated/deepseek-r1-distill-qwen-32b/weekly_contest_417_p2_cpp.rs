struct Solution;

impl Solution {
    const VOWEL_MASK: i32 = 1065233;

    fn f(word: &str, k: i32) -> u64 {
        let mut ans: u64 = 0;
        let mut cnt1 = [0; 26];
        let mut size1 = 0;
        let mut cnt2 = 0;
        let mut left = 0;

        for (right, c) in word.chars().enumerate() {
            let b = c as i32 - 'a' as i32;
            if (Self::VOWEL_MASK >> b) & 1 != 0 {
                if cnt1[b as usize] == 0 {
                    size1 += 1;
                }
                cnt1[b as usize] += 1;
            } else {
                cnt2 += 1;
            }

            while size1 == 5 && cnt2 >= k {
                let out = word[left] as i32 - 'a' as i32;
                if (Self::VOWEL_MASK >> out) & 1 != 0 {
                    cnt1[out as usize] -= 1;
                    if cnt1[out as usize] == 0 {
                        size1 -= 1;
                    }
                } else {
                    cnt2 -= 1;
                }
                left += 1;
            }

            ans += left as u64;
        }

        ans
    }

    fn count_of_substrings(word: &str, k: i32) -> u64 {
        Self::f(word, k) - Self::f(word, k + 1)
    }
}

fn main() {
    let word_size = std::io::stdin()
        .read_line()
        .expect("Failed to read wordSize")
        .trim()
        .parse::<usize>()
        .expect("Invalid wordSize");

    let word = std::io::stdin()
        .read_line()
        .expect("Failed to read word")
        .trim();

    let k = std::io::stdin()
        .read_line()
        .expect("Failed to read k")
        .trim()
        .parse::<i32>()
        .expect("Invalid k");

    println!("{}", Solution::count_of_substrings(word, k));
}