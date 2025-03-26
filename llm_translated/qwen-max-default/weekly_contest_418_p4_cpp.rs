use std::io::{self, BufRead, Write};

fn gcd_values(nums: &Vec<i32>, queries: &Vec<i64>) -> Vec<usize> {
    let mx = *nums.iter().max().unwrap_or(&0);
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

    let mut ans = Vec::with_capacity(queries.len());
    for &query in queries {
        let idx = cnt_gcd.binary_search(&(query + 1)).unwrap_or_else(|x| x);
        ans.push(idx);
    }
    ans
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stdin_lock = stdin.lock();
    let mut n = String::new();
    stdin_lock.read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");

    let mut nums = Vec::with_capacity(n);
    for _ in 0..n {
        let mut num = String::new();
        stdin_lock.read_line(&mut num).expect("Failed to read line");
        nums.push(num.trim().parse().expect("Please type a number!"));
    }

    let mut q = String::new();
    stdin_lock.read_line(&mut q).expect("Failed to read line");
    let q: usize = q.trim().parse().expect("Please type a number!");

    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let mut query = String::new();
        stdin_lock.read_line(&mut query).expect("Failed to read line");
        queries.push(query.trim().parse().expect("Please type a number!"));
    }

    let solution = gcd_values(&nums, &queries);
    for x in solution {
        write!(stdout, "{} ", x).expect("Failed to write to stdout");
    }
    writeln!(stdout).expect("Failed to write newline to stdout");
}