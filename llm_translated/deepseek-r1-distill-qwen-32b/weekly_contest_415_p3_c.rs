const MOD: i64 = 1070777777;

fn main() {
    use std::io;
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::Rng;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines().peekable();

    let words_size: usize = lines.next().unwrap().parse().unwrap();
    let mut words = Vec::with_capacity(words_size);
    for _ in 0..words_size {
        let len: usize = lines.next().unwrap().parse().unwrap();
        let word = lines.next().unwrap().chars().take(len).collect::<String>();
        words.push(word);
    }

    let target_size: usize = lines.next().unwrap().parse().unwrap();
    let target = lines.next().unwrap().chars().take(target_size).collect::<String>();

    let result = min_valid_strings(&words, &target);
    println!("{}", result);
}

fn min_valid_strings(words: &[String], target: &str) -> i32 {
    use std::collections::VecDeque;
    use std::cmp::Ordering;
    use rand::Rng;

    let n = target.len();
    if n == 0 {
        return 0;
    }

    let max_len = words.iter().map(|s| s.len()).max().unwrap_or(0);

    let mut rng = rand::thread_rng();
    let base = 800_000_000 + rng.gen_range(0..100_000_000) as u64;

    let mut pow_base = vec![1; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base) % MOD;
    }

    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        pre_hash[i + 1] = (pre_hash[i] * base + (target.as_bytes()[i] as u64)) % MOD;
    }

    let mut sets = vec![Vec::new(); max_len];
    for word in words {
        let len = word.len().min(max_len);
        let mut h = 0;
        for j in 0..len {
            h = (h * base + (word.as_bytes()[j] as u64)) % MOD;
            if j >= sets.len() {
                continue;
            }
            sets[j].push(h as i32);
        }
    }

    for set in &mut sets {
        set.sort();
        let mut unique = Vec::new();
        for num in set {
            if unique.last() != Some(num) {
                unique.push(num);
            }
        }
        *set = unique;
    }

    let mut ans = 0;
    let mut cur_r = 0;
    let mut nxt_r = 0;

    while cur_r < n {
        while nxt_r < n && (nxt_r - cur_r) < max_len as usize {
            let prefix_len = nxt_r - cur_r;
            if prefix_len >= sets.len() {
                break;
            }
            let current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[cur_r] * pow_base[prefix_len + 1] % MOD)) % MOD;
            let sh = current_hash as i32;

            let set = &sets[prefix_len];
            let mut left = 0;
            let mut right = set.len() as i32 - 1;
            let mut found = false;
            while left <= right {
                let mid = (left + right) / 2;
                match set[mid as usize].cmp(&sh) {
                    Ordering::Equal => {
                        found = true;
                        break;
                    }
                    Ordering::Less => left = mid + 1,
                    Ordering::Greater => right = mid - 1,
                }
            }
            if found {
                nxt_r += 1;
            } else {
                break;
            }
        }

        if cur_r == nxt_r {
            return -1;
        }
        cur_r = nxt_r;
        ans += 1;
    }

    ans
}