use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

// 频率和值对
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Pair {
    val: i32,   // 元素值
    freq: i32,  // 频率
}

// 比较函数: 优先按频率降序，然后按值降序
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
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for j in 0..k {
            *counts.entry(nums[i + j]).or_insert(0) += 1;
        }

        let mut active_elements: Vec<Pair> = counts
            .iter()
            .map(|(&val, &freq)| Pair { val, freq })
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let k: i32 = iter.next().unwrap().parse().unwrap();
    let x: i32 = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums_size: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = find_x_sum(&nums, k, x);

    for &val in &result {
        print!("{} ", val);
    }
    println!();
}