use std::io;

fn get(l: usize, r: usize, pre_count: &Vec<Vec<i32>>, count: &[i32]) -> usize {
    let mut border = l;
    let mut left = l;
    let mut right = r;
    while left < right {
        let mid = (left + right) >> 1;
        let mut found = true;
        for i in 0..26 {
            if pre_count[mid][i] - pre_count[border - 1][i] < count[i] {
                found = false;
                break;
            }
        }
        if found {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn valid_substring_count(word1: &str, word2: &str) -> i64 {
    let mut count = [0; 26];
    for c in word2.chars() {
        count[c as usize - 'a' as usize] += 1;
    }

    let n = word1.len();
    let mut pre_count = vec![vec![0; 26]; n + 1];
    for i in 1..=n {
        pre_count[i] = pre_count[i - 1].clone();
        let c = word1.chars().nth(i - 1).unwrap();
        pre_count[i][c as usize - 'a' as usize] += 1;
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
    let word1 = lines.next().unwrap();
    let len2: usize = lines.next().unwrap().parse().unwrap();
    let word2 = lines.next().unwrap();
    
    let result = valid_substring_count(word1, word2);
    println!("{}", result);
    
    Ok(())
}