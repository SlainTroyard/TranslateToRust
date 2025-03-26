const INF: i32 = 1_000_000;

fn dfs(dp: &mut [i32; 26], a: &[i32; 26], k: i32, i: usize, size: usize) -> i32 {
    if i >= size {
        return 0;
    }
    if dp[i] != -1 {
        return dp[i];
    }

    let mut ans = INF;
    if a[i] >= k {
        let more = a[i] - k;
        ans = std::cmp::min(ans, more + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            let sum = a[i + 1] + more;
            if sum >= k {
                ans = std::cmp::min(ans, more + dfs(dp, a, k, i + 2, size));
            } else {
                ans = std::cmp::min(ans, more + (k - sum) + dfs(dp, a, k, i + 2, size));
            }
        }
    } else {
        ans = std::cmp::min(ans, (k - a[i]) + dfs(dp, a, k, i + 1, size));
        if i + 1 < size && a[i + 1] <= k {
            let sum = a[i] + a[i + 1];
            if sum >= k {
                ans = std::cmp::min(ans, a[i] + dfs(dp, a, k, i + 2, size));
            } else {
                ans = std::cmp::min(ans, a[i] + (k - sum) + dfs(dp, a, k, i + 2, size));
            }
        }
    }

    dp[i] = ans;
    ans
}

fn make_string_good(s: &str) -> i32 {
    let n = s.len();
    if n == 0 {
        return 0;
    }

    let mut a = [0; 26];
    for c in s.chars() {
        let index = (c as usize) - ('a' as usize);
        a[index] += 1;
    }

    let mut ans = n;
    for i in 0..26 {
        ans = std::cmp::min(ans, n - a[i]);
    }

    for k in 1..=n {
        let mut dp = [-1; 26];
        let current = dfs(&mut dp, &a, k as i32, 0, 26);
        ans = std::cmp::min(ans, current);
    }

    ans
}

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read input");
    let s = s.trim();
    let result = make_string_good(s);
    println!("{}", result);
}