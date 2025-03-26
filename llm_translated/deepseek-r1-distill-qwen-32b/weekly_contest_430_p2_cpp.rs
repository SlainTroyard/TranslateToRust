struct Solution;

impl Solution {
    fn answer_string(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }
        let n = s.len();
        let mut ans = String::new();
        for i in 0..n {
            let max_val = std::cmp::max(k - 1, i as i32);
            let len = n as i32 - max_val;
            if len <= 0 {
                continue;
            }
            let end = i + len as usize;
            let end = if end > n { n } else { end };
            let substr = &s[i..end];
            if substr > ans {
                ans = substr.to_string();
            }
        }
        ans
    }
}

fn main() -> std::io::Result<()> {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() < 2 {
        eprintln!("Not enough input");
        return Ok(());
    }

    let s = tokens[0].to_string();
    let k = tokens[1].parse::<i32>().unwrap();

    let result = Solution::answer_string(s, k);
    println!("{}", result);

    Ok(())
}