use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

struct LCG {
    state: u32,
}

impl LCG {
    fn new(seed: u32) -> Self {
        LCG { state: seed }
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
}

const MOD: u64 = 1070777777;

fn min_valid_strings(words: Vec<String>, target: &str) -> i32 {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let mut rng = LCG::new(seed);
    let random_part = rng.next() % 100_000_000;
    let base = 800_000_000 + random_part as u32;

    let n = target.len();
    let mut pow_base = vec![0u64; n + 1];
    pow_base[0] = 1;
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] * base as u64) % MOD;
    }

    let mut pre_hash = vec![0u64; n + 1];
    for i in 0..n {
        let c = target.as_bytes()[i] as u64;
        pre_hash[i + 1] = (pre_hash[i] * base as u64 + c) % MOD;
    }

    let max_len = words.iter().map(|word| word.len()).max().unwrap_or(0);
    if max_len == 0 {
        return -1;
    }

    let mut sets: Vec<Vec<i32>> = vec![Vec::new(); max_len];
    for word in words.iter() {
        let actual_len = word.len();
        let len = actual_len.min(max_len);
        let mut h: u64 = 0;
        for j in 0..len {
            let c = word.as_bytes()[j] as u64;
            h = (h * base as u64 + c) % MOD;
            sets[j].push(h as i32);
        }
    }

    for set in sets.iter_mut() {
        set.sort_unstable();
        set.dedup();
    }

    let mut ans = 0;
    let (mut cur_r, mut nxt_r) = (0, 0);
    for i in 0..n {
        while nxt_r < n && (nxt_r as i32 - i as i32) < max_len as i32 {
            if (nxt_r + 1) as usize > n {
                break;
            }
            let length = (nxt_r + 1 - i) as usize;
            let pow = pow_base[length];
            let hash_start = pre_hash[i] as u64;
            let hash_end = pre_hash[nxt_r + 1] as u64;
            let pow_term = (hash_start * pow) % MOD;
            let current_hash = (hash_end - pow_term + MOD) % MOD;
            let current_hash_i32 = current_hash as i32;
            let prefix_len = (nxt_r - i) as usize;
            if prefix_len >= max_len || prefix_len < 0 {
                break;
            }
            if sets[prefix_len].binary_search(&current_hash_i32).is_ok() {
                nxt_r += 1;
            } else {
                break;
            }
        }
        if i == cur_r as usize {
            if i as i32 == nxt_r as i32 {
                return -1;
            }
            cur_r = nxt_r as i32;
            ans += 1;
        }
    }

    ans
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let words_size: usize = lines.next().unwrap().parse().unwrap();
    let mut words = Vec::new();

    for _ in 0..words_size {
        let _word_len = lines.next().unwrap(); // Read but ignore the length
        let word = lines.next().unwrap();
        words.push(word);
    }

    let _target_size = lines.next().unwrap(); // Read but ignore the target length
    let target = lines.next().unwrap();

    let result = min_valid_strings(words, &target);
    println!("{}", result);
}