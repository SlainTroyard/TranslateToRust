pub fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0i32; 26]; // Tracks difference between counts of characters in current window and word2
    for c in word2.bytes() {
        let idx = (c - b'a') as usize;
        diff[idx] -= 1; // Initialize with counts from word2
    }

    let mut cnt = 0; // Number of characters with non-zero difference
    for &d in &diff {
        if d < 0 {
            cnt += 1;
        }
    }

    let len1 = word1.len();
    let mut res = 0i64;
    let mut l = 0;
    let mut r = 0;

    while l < len1 {
        // Expand the window to the right as long as there are mismatches (cnt > 0)
        while r < len1 && cnt > 0 {
            let c = (word1.as_bytes()[r] - b'a') as usize;
            diff[c] += 1;
            if diff[c] == 0 {
                cnt -= 1; // Transition from negative to zero: reduce mismatch count
            }
            r += 1;
        }

        // If all characters match (cnt == 0), add valid substrings starting at current l
        if cnt == 0 {
            res += (len1 - r + 1) as i64;
        }

        // Move left pointer and adjust diff/cnt for the removed character
        let c = (word1.as_bytes()[l] - b'a') as usize;
        diff[c] -= 1;
        if diff[c] == -1 {
            cnt += 1; // Transition from zero to negative: increase mismatch count
        }
        l += 1;
    }

    res
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let len1 = tokens[0].parse::<usize>().expect("Invalid length");
    let word1 = tokens[1];
    let len2 = tokens[2].parse::<usize>().expect("Invalid length");
    let word2 = tokens[3];

    let result = valid_substring_count(word1, word2);
    println!("{}", result);
}