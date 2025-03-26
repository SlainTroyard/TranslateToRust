const MOD: i64 = 1_000_000_007;
const SIZE: usize = 26;

type Matrix = Vec<Vec<i64>>;

fn mul(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = vec![vec![0; SIZE]; SIZE];
    for i in 0..SIZE {
        for k in 0..SIZE {
            if a[i][k] == 0 {
                continue;
            }
            for j in 0..SIZE {
                c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    c
}

fn pow(mut a: Matrix, mut n: i64) -> Matrix {
    let mut res = identity_matrix();
    while n > 0 {
        if n % 2 == 1 {
            res = mul(&res, &a);
        }
        a = mul(&a, &a);
        n /= 2;
    }
    res
}

fn identity_matrix() -> Matrix {
    let mut mat = vec![vec![0; SIZE]; SIZE];
    for i in 0..SIZE {
        mat[i][i] = 1;
    }
    mat
}

fn main() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let s_len = tokens[0].parse::<i32>().unwrap();
    let s = tokens[1];
    let t = tokens[2].parse::<i64>().unwrap();
    let nums: Vec<i32> = tokens[3..29]
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut m = vec![vec![0; SIZE]; SIZE];
    for i in 0..SIZE {
        let max_j = i as i32 + nums[i];
        for j in (i as i32 + 1)..=max_j {
            let idx = (j % SIZE as i32) as usize;
            m[i][idx] = 1;
        }
    }

    let m_pow = pow(m, t);

    let mut cnt = [0; SIZE];
    for c in s.chars() {
        let idx = (c as usize) - ('a' as usize);
        cnt[idx] += 1;
    }

    let mut ans = 0;
    for i in 0..SIZE {
        let row_sum: i64 = m_pow[i].iter().sum();
        ans = (ans + row_sum * cnt[i] as i64) % MOD;
    }

    println!("{}", ans % MOD);
}