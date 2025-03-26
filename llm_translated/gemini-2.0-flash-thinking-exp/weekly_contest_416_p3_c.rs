use std::io;

fn get(l: i32, r: i32, pre_count: &[[i32; 26]], count: &[i32; 26]) -> i32 {
    let mut border = l;
    let mut l_mut = l;
    let mut r_mut = r;
    while l_mut < r_mut {
        let m = (l_mut + r_mut) >> 1;
        let mut f = true;
        for i in 0..26 {
            if pre_count[m as usize][i] - pre_count[(border - 1) as usize][i] < count[i] {
                f = false;
                break;
            }
        }
        if f {
            r_mut = m;
        } else {
            l_mut = m + 1;
        }
    }
    l_mut
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0; 26];
    for byte in word2.bytes() {
        count[(byte - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![[0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1];
        pre_count[i][(word1.as_bytes()[i - 1] - b'a') as usize] += 1;
    }

    let mut res: i64 = 0;
    for l in 1..=n as i32 {
        let r = get(l, n as i32 + 1, &pre_count, &count);
        res += (n as i32 - r + 1) as i64;
    }
    res
}

fn main() {
    let mut len1_str = String::new();
    io::stdin().read_line(&mut len1_str).expect("Failed to read line");
    let len1: i32 = len1_str.trim().parse().expect("Invalid input for len1");

    let mut word1 = String::new();
    io::stdin().read_line(&mut word1).expect("Failed to read line");
    let word1 = word1.trim();

    let mut len2_str = String::new();
    io::stdin().read_line(&mut len2_str).expect("Failed to read line");
    let len2: i32 = len2_str.trim().parse().expect("Invalid input for len2");

    let mut word2 = String::new();
    io::stdin().read_line(&mut word2).expect("Failed to read line");
    let word2 = word2.trim();

    println!("{}", valid_substring_count(word1, word2));
}