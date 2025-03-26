use std::cmp::max;
use std::io;
use std::io::Read;

fn max_possible_score(start: &mut [i32], d: i32) -> i32 {
    // Sort array
    start.sort_unstable();

    // Binary search
    let mut left: i64 = 0;
    let right: i64 = ((start[start.len() - 1] as i64 + d as i64 - start[0] as i64) / (start.len() as i64 - 1) + 1) as i64;

    let mut left_mut = left;
    let mut right_mut = right;

    while left_mut + 1 < right_mut {
        let mid: i64 = left_mut + (right_mut - left_mut) / 2;

        // Check function
        let mut sum: i64 = i64::min_value();
        let mut valid: bool = true;
        for i in 0..start.len() {
            sum = max(sum + mid, start[i] as i64);
            if sum > start[i] as i64 + d as i64 {
                valid = false;
                break;
            }
        }

        if valid {
            left_mut = mid;
        } else {
            right_mut = mid;
        }
    }
    left_mut as i32
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut split = first_line.split_whitespace();
    let start_size: i32 = split.next().unwrap().parse().unwrap();
    let d: i32 = split.next().unwrap().parse().unwrap();

    let mut start: Vec<i32> = Vec::new();
    for _ in 0..start_size {
        let line = lines.next().unwrap();
        let num: i32 = line.parse().unwrap();
        start.push(num);
    }

    println!("{}", max_possible_score(&mut start, d));

    Ok(())
}