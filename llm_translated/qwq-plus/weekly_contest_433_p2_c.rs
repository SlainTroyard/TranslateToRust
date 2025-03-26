const MOD: u64 = 1_000_000_007;
const MX: usize = 100_000;

fn power(mut x: u64, mut n: u64, modu: u64) -> u64 {
    let mut res = 1;
    while n > 0 {
        if n % 2 == 1 {
            res = res * x % modu;
        }
        x = x * x % modu;
        n /= 2;
    }
    res
}

fn comb(n: usize, m: usize, F: &[u64], inv_F: &[u64]) -> u64 {
    if m > n {
        return 0;
    }
    let numerator = F[n];
    let denom = inv_F[m] * inv_F[n - m] % MOD;
    (numerator * denom) % MOD
}

fn compute_result(nums: &mut [i32], k: usize, F: &[u64], inv_F: &[u64]) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans: u64 = 0;
    let mut s: u64 = 1;
    for i in 0..n {
        let term = (nums[i] as u64) + (nums[n - 1 - i] as u64);
        ans = (ans + s * term) % MOD;
        let c = comb(i, k - 1, F, inv_F);
        s = (s * 2 - c + MOD) % MOD;
    }
    ans as i32
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let k: usize = parts.next().unwrap().parse().unwrap();

    let second_line = lines.next().unwrap();
    let nums: Vec<i32> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    assert_eq!(nums.len(), n);

    // Precompute F and inv_F
    let mut F = vec![1; MX];
    for i in 1..MX {
        F[i] = (F[i - 1] * i as u64) % MOD;
    }

    let mut inv_F = vec![0; MX];
    inv_F[MX - 1] = power(F[MX - 1], MOD - 2, MOD);
    for i in (1..MX).rev() {
        inv_F[i - 1] = inv_F[i] * (i as u64) % MOD;
    }

    // Compute the result
    let mut nums_clone = nums;
    let result = compute_result(&mut nums_clone, k, &F, &inv_F);
    println!("{}", result);
}