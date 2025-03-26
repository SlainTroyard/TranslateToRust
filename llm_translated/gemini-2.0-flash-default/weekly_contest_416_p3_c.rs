use std::io;
use std::io::Read;

fn get(l: usize, r: usize, pre_count: &[[i32; 26]], count: &[i32; 26]) -> usize {
    let mut left = l;
    let mut right = r;
    let border = l;

    while left < right {
        let m = (left + right) >> 1;
        let mut f = true;
        for i in 0..26 {
            if pre_count[m][i] - pre_count[border - 1][i] < count[i] {
                f = false;
                break;
            }
        }
        if f {
            right = m;
        } else {
            left = m + 1;
        }
    }
    left
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0; 26];
    for &b in word2.as_bytes() {
        count[(b - b'a') as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![[0; 26]; n + 1];

    for i in 1..=n {
        pre_count[i] = pre_count[i - 1];
        pre_count[i][(word1.as_bytes()[i - 1] - b'a') as usize] += 1;
    }

    let mut res = 0;
    for l in 1..=n {
        let r = get(l, n + 1, &pre_count, &count);
        res += (n - r + 1) as i64;
    }
    res
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let len1: usize = lines.next().unwrap().parse().unwrap();
    let word1: String = lines.next().unwrap().to_string();
    let len2: usize = lines.next().unwrap().parse().unwrap();
    let word2: String = lines.next().unwrap().to_string();

    println!("{}", valid_substring_count(&word1, &word2));

    Ok(())
}