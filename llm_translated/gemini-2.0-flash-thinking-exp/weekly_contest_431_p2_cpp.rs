use std::io;
use std::io::BufRead;

fn calculate_score(s: &str) -> i64 {
    let mut stk: Vec<Vec<usize>> = vec![Vec::new(); 26];
    let mut ans: i64 = 0;
    for (i, char_s) in s.chars().enumerate() {
        let c = (char_s as u32 - 'a' as u32) as usize;
        if !stk[25 - c].is_empty() {
            if let Some(top_index) = stk[25 - c].pop() {
                ans += (i as i64 - top_index as i64);
            }
        } else {
            stk[c].push(i);
        }
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.lock().read_line(&mut s).expect("Failed to read line");
    let s = s.trim(); // Remove trailing newline

    let result = calculate_score(s);
    println!("{}", result);
}