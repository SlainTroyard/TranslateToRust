use std::io::{self, BufRead, Write};

fn gcd_values(nums: &Vec<i32>, queries: &Vec<i64>) -> Vec<usize> {
    let mx = *nums.iter().max().unwrap();
    let mut cnt_x = vec![0; (mx + 1) as usize];
    for &x in nums {
        cnt_x[x as usize] += 1;
    }

    let mut cnt_gcd = vec![0; (mx + 1) as usize];
    for i in (1..=mx).rev() {
        let mut c = 0;
        for j in (i..=mx).step_by(i as usize) {
            c += cnt_x[j as usize];
            cnt_gcd[i as usize] -= cnt_gcd[j as usize];
        }
        cnt_gcd[i as usize] += (c * (c - 1)) / 2;
    }
    for i in 1..=mx {
        cnt_gcd[i as usize] += cnt_gcd[(i - 1) as usize];
    }

    let mut ans = vec![0; queries.len()];
    for (i, &query) in queries.iter().enumerate() {
        let pos = cnt_gcd.binary_search(&(query + 1)).unwrap_or_else(|x| x);
        ans[i] = pos;
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut n = String::new();
    stdin_lock.read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Failed to parse n");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num_str = String::new();
        stdin_lock.read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Failed to parse num");
        nums.push(num);
    }

    let mut q = String::new();
    stdin_lock.read_line(&mut q).expect("Failed to read line");
    let q: usize = q.trim().parse().expect("Failed to parse q");

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let mut query_str = String::new();
        stdin_lock.read_line(&mut query_str).expect("Failed to read line");
        let query: i64 = query_str.trim().parse().expect("Failed to parse query");
        queries.push(query);
    }

    let solution = gcd_values(&nums, &queries);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for &x in &solution {
        write!(handle, "{} ", x).expect("Failed to write to stdout");
    }
    writeln!(handle).expect("Failed to write newline to stdout");
}