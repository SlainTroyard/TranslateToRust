use std::io;

fn max_frequency(nums: &[i32], k: i32) -> i32 {
    let mut f0 = 0;
    let mut f1 = [0; 51];
    let mut max_f1 = 0;
    let mut f2 = 0;

    for &x in nums {
        f2 = std::cmp::max(f2, max_f1) + (x == k) as i32;

        let x_usize = x as usize;
        f1[x_usize] = std::cmp::max(f1[x_usize], f0) + 1;

        if x == k {
            f0 += 1;
        }

        if f1[x_usize] > max_f1 {
            max_f1 = f1[x_usize];
        }
    }

    std::cmp::max(max_f1, f2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let mut tokens = input.split_whitespace();
    let n = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid n");
    let k = tokens.next().and_then(|s| s.parse().ok()).expect("Invalid k");

    let nums: Vec<i32> = tokens
        .take(n as usize)
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let result = max_frequency(&nums, k);
    println!("{}", result);
}