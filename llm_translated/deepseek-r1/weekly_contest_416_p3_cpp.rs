fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0; 26];
    for c in word2.chars() {
        let idx = (c as u8 - b'a') as usize;
        count[idx] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![vec![0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i-1].clone();
        let c = word1.as_bytes()[i-1];
        let idx = (c - b'a') as usize;
        pre_count[i][idx] += 1;
    }

    let mut res = 0i64;
    for l in 1..=n {
        let mut left = l;
        let mut right = n + 1;

        while left < right {
            let mid = (left + right) / 2;
            let mut valid = true;

            for i in 0..26 {
                if pre_count[mid][i] - pre_count[l-1][i] < count[i] {
                    valid = false;
                    break;
                }
            }

            if valid {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        res += (n as i64) - (left as i64) + 1;
    }

    res
}

fn main() {
    let input = {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
        input
    };
    let mut tokens = input.split_whitespace();

    let _len1: usize = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid len1");
    let word1 = tokens.next().expect("Missing word1");

    let _len2: usize = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid len2");
    let word2 = tokens.next().expect("Missing word2");

    let result = valid_substring_count(word1, word2);
    println!("{}", result);
}