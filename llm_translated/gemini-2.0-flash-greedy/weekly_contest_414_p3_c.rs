use std::io;
use std::io::Read;

fn find_maximum_score(nums: &[i32]) -> i64 {
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let mut r: usize = 1;

    while r < nums.len() {
        if nums[l] < nums[r] {
            ans += (r - l) as i64 * nums[l] as i64;
            l = r;
        }
        r += 1;
    }

    ans + (r - l - 1) as i64 * nums[l] as i64
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();
    let nums_size: usize = lines.next().unwrap().parse()?;

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), nums_size);

    let result = find_maximum_score(&nums);
    println!("{}", result);

    Ok(())
}