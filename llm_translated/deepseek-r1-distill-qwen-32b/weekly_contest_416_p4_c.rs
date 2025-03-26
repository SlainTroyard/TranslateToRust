fn update(diff: &mut [i32; 26], c: usize, add: i32, cnt: &mut i32) {
    diff[c] += add;
    if add == 1 && diff[c] == 0 {
        *cnt -= 1;
    } else if add == -1 && diff[c] == -1 {
        *cnt += 1;
    }
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut diff = [0; 26];
    let word2_indices: Vec<usize> = word2.chars()
        .map(|c| (c as usize) - ('a' as usize))
        .collect();
    for c in word2_indices {
        diff[c] -= 1;
    }

    let mut cnt = 0;
    for i in 0..26 {
        if diff[i] < 0 {
            cnt += 1;
        }
    }

    let word1_indices: Vec<usize> = word1.chars()
        .map(|c| (c as usize) - ('a' as usize))
        .collect();
    let len1 = word1_indices.len();
    let mut l = 0;
    let mut r = 0;
    let mut res = 0;

    while l < len1 {
        while r < len1 && cnt > 0 {
            let c = word1_indices[r];
            update(&mut diff, c, 1, &mut cnt);
            r += 1;
        }
        if cnt == 0 {
            res += (len1 - r) as i64 + 1;
        }
        let c = word1_indices[l];
        update(&mut diff, c, -1, &mut cnt);
        l += 1;
    }

    res
}

fn main() {
    use std::io;
    use std::str;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let _len1: usize = parts.next().unwrap().parse().unwrap();
    let word1 = parts.next().unwrap();
    let _len2: usize = parts.next().unwrap().parse().unwrap();
    let word2 = parts.next().unwrap();

    let result = valid_substring_count(word1, word2);
    println!("{}", result);
}