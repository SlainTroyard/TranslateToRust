use std::io;

fn merge(intervals: &[(usize, usize)], len: usize) -> Vec<(usize, usize)> {
    if intervals.is_empty() {
        return Vec::new();
    }

    let mut merged = Vec::new();
    let (mut current_start, mut current_end) = intervals[0];

    for &(start, end) in intervals.iter().skip(1) {
        if start <= current_end && (start - current_start + 1) <= len {
            current_end = current_end.max(end);
        } else {
            merged.push((current_start, current_end));
            current_start = start;
            current_end = end;
        }
    }

    merged.push((current_start, current_end));
    merged
}

fn is_poss(s: &[char], allowed_ops: usize, mid: usize) -> bool {
    let n = s.len();
    let mut i = 0;
    let mut zero = 0;
    let mut one = 0;
    let mut intervals = Vec::new();

    for j in 0..n {
        match s[j] {
            '0' => zero += 1,
            _ => one += 1,
        }

        while (j - i + 1) > mid {
            match s[i] {
                '0' => zero -= 1,
                _ => one -= 1,
            }
            i += 1;
        }

        if (j - i + 1) == mid {
            if zero == 0 || one == 0 {
                intervals.push((i, j));
            }
        }
    }

    let merged = merge(&intervals, mid);
    merged.len() <= allowed_ops
}

fn get_mini(s: &[char], even: char, odd: char) -> usize {
    s.iter()
        .enumerate()
        .filter(|&(i, &c)| {
            if i % 2 == 0 {
                c != even
            } else {
                c != odd
            }
        })
        .count()
}

fn min_length(s: &[char], num_ops: usize) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }

    let op_even_odd = get_mini(s, '0', '1');
    let op_odd_even = get_mini(s, '1', '0');
    let min_op = op_even_odd.min(op_odd_even);

    if min_op <= num_ops {
        return 1;
    }

    let mut start = 3;
    let mut end = n;
    let mut res = 2;

    while start <= end {
        let mid = start + (end - start) / 2;
        if is_poss(s, num_ops, mid) {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read string");
    let s: Vec<char> = input.trim().chars().collect();

    let mut ops_input = String::new();
    io::stdin().read_line(&mut ops_input).expect("Failed to read operations");
    let num_ops: usize = ops_input.trim().parse().expect("Invalid number");

    println!("{}", min_length(&s, num_ops));
}