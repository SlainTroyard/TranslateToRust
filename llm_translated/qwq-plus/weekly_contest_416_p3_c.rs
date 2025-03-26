use std::io;

fn valid_substring_count(word1: &str, word2: &str) -> u64 {
    let n = word1.len();
    let mut count = [0; 26];
    for &c in word2.as_bytes() {
        let idx = (c - b'a') as usize;
        count[idx] += 1;
    }

    let mut pre_count = vec![vec![0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1].clone();
        let c = word1.as_bytes()[i - 1];
        let idx = (c - b'a') as usize;
        pre_count[i][idx] += 1;
    }

    let mut res = 0;
    let get = |l: usize, r_initial: usize| -> usize {
        let mut l_b = l;
        let mut r = r_initial;
        let border = l;
        while l_b < r {
            let m = (l_b + r) / 2;
            let mut f = true;
            for i in 0..26 {
                let pre_m = pre_count[m][i];
                let pre_border = pre_count[border - 1][i];
                if (pre_m - pre_border) < count[i] {
                    f = false;
                    break;
                }
            }
            if f {
                r = m;
            } else {
                l_b = m + 1;
            }
        }
        l_b
    };

    for l in 1..=n {
        let r = get(l, n + 1);
        res += (n - r + 1) as u64;
    }
    res
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 4 {
        return Err("Not enough tokens".into());
    }

    let len1 = tokens[0].parse::<usize>().unwrap();
    let word1 = tokens[1];
    let len2 = tokens[2].parse::<usize>().unwrap();
    let word2 = tokens[3];

    if word1.len() != len1 || word2.len() != len2 {
        return Err("Input lengths do not match".into());
    }

    let result = valid_substring_count(word1, word2);
    println!("{}", result);
    Ok(())
}