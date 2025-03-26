use std::io;

fn calculate_score(s: &str) -> i64 {
    let len = s.len();
    let mut stacks: Vec<Vec<usize>> = (0..26).map(|_| Vec::with_capacity(len)).collect();
    let mut ans = 0i64;

    for (i, c_char) in s.chars().enumerate() {
        let c = (c_char as u8 - b'a') as usize;
        let opposite = 25 - c;
        if let Some(popped) = stacks[opposite].pop() {
            ans += (i as i64) - (popped as i64);
        } else {
            stacks[c].push(i);
        }
    }
    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)
        .expect("Failed to read input");
    // Extract the first token, ignoring leading/trailing whitespace
    let s = input.split_whitespace().next().unwrap_or("").to_string();
    let ans = calculate_score(&s);
    println!("{}", ans);
}