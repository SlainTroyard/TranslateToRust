use std::io;
use std::io::Read;

fn max_distinct_elements(arr: &mut [i32], diff: i32) -> i32 {
    let mut prev: i32 = i32::MIN;
    let mut distinct_count: i32 = 0;

    arr.sort_unstable();

    for &val in arr.iter() {
        let x: i32 = if prev.saturating_add(1) > val.saturating_sub(diff) {
            prev.saturating_add(1)
        } else {
            val.saturating_sub(diff)
        };

        if x <= val.saturating_add(diff) {
            distinct_count += 1;
            prev = x;
        }
    }

    distinct_count
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let diff: i32 = lines.next().unwrap().parse().unwrap();

    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut arr_mut = arr.clone(); // Create a mutable copy
    let result: i32 = max_distinct_elements(&mut arr_mut, diff);

    println!("{}", result);

    Ok(())
}