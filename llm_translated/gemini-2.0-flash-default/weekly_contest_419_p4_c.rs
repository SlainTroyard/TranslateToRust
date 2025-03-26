use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Pair {
    val: i32,
    freq: i32,
}

impl Pair {
    fn new(val: i32, freq: i32) -> Self {
        Pair { val, freq }
    }
}

fn cmp(a: &Pair, b: &Pair) -> Ordering {
    if b.freq != a.freq {
        b.freq.cmp(&a.freq)
    } else {
        b.val.cmp(&a.val)
    }
}

fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i64> {
    let nums_size = nums.len();
    let k = k as usize;
    let x = x as usize;

    let mut result = Vec::with_capacity(nums_size - k + 1);

    for i in 0..=nums_size - k {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for j in 0..k {
            let val = nums[i + j];
            *freq_map.entry(val).or_insert(0) += 1;
        }

        let mut active_elements: Vec<Pair> = freq_map
            .iter()
            .map(|(&val, &freq)| Pair::new(val, freq))
            .collect();

        active_elements.sort_by(cmp);

        let count_to_sum = std::cmp::min(active_elements.len(), x);
        let mut sum: i64 = 0;
        for j in 0..count_to_sum {
            sum += (active_elements[j].val as i64) * (active_elements[j].freq as i64);
        }
        result.push(sum);
    }

    result
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let k: i32 = parts.next().unwrap().parse().unwrap();
    let x: i32 = parts.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let nums_size: usize = second_line.parse().unwrap();

    let third_line = lines.next().unwrap();
    let nums: Vec<i32> = third_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(nums.len(), nums_size);

    let result = find_x_sum(&nums, k, x);

    for &val in &result {
        print!("{} ", val);
    }
    println!();

    Ok(())
}