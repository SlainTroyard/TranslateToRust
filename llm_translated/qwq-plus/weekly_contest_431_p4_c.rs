use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct DpNode {
    max: [i64; 4],
    idx: [u64; 4],
}

fn insert_index(idx: u64, i: usize, count: usize) -> u64 {
    let mut value = [0u16; 4];
    value[0] = ((idx >> 48) as u16);
    value[1] = ((idx >> 32 & 0xFFFF) as u16);
    value[2] = ((idx >> 16 & 0xFFFF) as u16);
    let i = i as u16;
    value[count] = i;
    let mut x = count as isize - 1;
    while x >= 0 {
        let x_usize = x as usize;
        if value[x_usize] > i {
            value.swap(x_usize + 1, x_usize);
            x -= 1;
        } else {
            break;
        }
    }
    let result = (value[0] as u64) << 48
        | (value[1] as u64) << 32
        | (value[2] as u64) << 16
        | (value[3] as u64);
    result
}

fn maximum_weight(intervals: &[[i32; 3]], return_size: &mut usize) -> Vec<i32> {
    let n = intervals.len();
    let mut dp = vec![DpNode { max: [0; 4], idx: [0; 4] }; n];
    let mut last = DpNode { max: [0; 4], idx: [0; 4] };

    let mut left_queue = BinaryHeap::new();
    let mut right_queue = BinaryHeap::new();

    for (i, interval) in intervals.iter().enumerate() {
        let start = interval[0] as u32;
        let end = interval[1] as u32;
        left_queue.push(Reverse((start, i)));
        right_queue.push(Reverse((end, i)));
    }

    let mut max = 0i64;
    let mut sel = 0u64;

    while let Some(Reverse((edge, i))) = left_queue.pop() {
        loop {
            if let Some(&Reverse((end_val, j))) = right_queue.peek() {
                if edge > end_val {
                    right_queue.pop();
                    for k in 0..3 {
                        if last.max[k] < dp[j].max[k] || (last.max[k] == dp[j].max[k] && last.idx[k] > dp[j].idx[k]) {
                            last.max[k] = dp[j].max[k];
                            last.idx[k] = dp[j].idx[k];
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        dp[i] = last;

        for k in 0..4 {
            let prev = if k == 0 { 0 } else { dp[i].max[k - 1] };
            if k > 0 && prev <= 0 {
                continue;
            }

            let mut idx: u64 = 0;
            if k == 0 {
                idx = (i as u64) << 48;
            } else {
                idx = insert_index(dp[i].idx[k - 1], i, k);
            }

            let new_max = intervals[i][2] as i64 + prev;
            if dp[i].max[k] < new_max || (dp[i].max[k] == new_max && dp[i].idx[k] > idx) {
                dp[i].max[k] = new_max;
                dp[i].idx[k] = idx;
            }

            if max < dp[i].max[k] || (max == dp[i].max[k] && sel > dp[i].idx[k]) {
                *return_size = k + 1;
                max = dp[i].max[k];
                sel = dp[i].idx[k];
            }
        }
    }

    let mut result = Vec::with_capacity(*return_size);
    for i in 0..*return_size {
        let shift = (3 - i) << 4;
        let val = (sel >> shift) as u16 as i32;
        result.push(val);
    }
    result
}

fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut intervals = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        intervals.push([nums[0], nums[1], nums[2]]);
    }

    let mut return_size = 0;
    let result = maximum_weight(&intervals, &mut return_size);

    for &val in &result {
        print!("{} ", val);
    }
    println!();
}