use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
extern crate rand;
use rand::Rng;
use rand::rngs::StdRng;

const MOD: u64 = 1070777777;

struct HashSet {
    data: Vec<i32>,
}

impl HashSet {
    fn new() -> Self {
        HashSet { data: Vec::new() }
    }

    fn add(&mut self, hash: i32) {
        self.data.push(hash);
    }

    fn sort_and_unique(&mut self) {
        self.data.sort_unstable();
        self.data.dedup();
    }
}

fn main() {
    let mut tokens = Vec::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for token in input.split_whitespace() {
        tokens.push(token.to_string());
    }

    let mut token_iter = tokens.iter();
    let words_size: usize = token_iter.next().unwrap().parse().unwrap();

    let mut words: Vec<String> = Vec::new();
    let mut words_lengths: Vec<usize> = Vec::new();
    for _ in 0..words_size {
        let len_str = token_iter.next().unwrap();
        let len: usize = len_str.parse().unwrap();
        let word = token_iter.next().unwrap().clone();
        words.push(word);
        words_lengths.push(len);
    }

    let target_size_str = token_iter.next().unwrap();
    let target_size: usize = target_size_str.parse().unwrap();
    let target = token_iter.next().unwrap().clone();

    let n = target.len();
    if n == 0 {
        println!("0");
        return;
    }

    let max_len = *words_lengths.iter().max().unwrap_or(&0);
    if max_len == 0 {
        println!("-1");
        return;
    }

    // Seed the RNG
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64;
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate base between 8e8 and 8e8 + 32767 (assuming C's RAND_MAX is 32767)
    let random_part = rng.gen_range(0..32768);
    let base = 800_000_000u32 + random_part as u32;

    // Precompute pow_base
    let mut pow_base = vec![1; n + 1];
    for i in 0..n {
        pow_base[i + 1] = (pow_base[i] as u64 * base as u64 % MOD) % MOD;
    }

    // Compute pre_hash for target
    let mut pre_hash = vec![0; n + 1];
    for i in 0..n {
        let c = target.as_bytes()[i] as u64;
        pre_hash[i + 1] = (pre_hash[i] * base as u64 + c) % MOD;
    }

    // Initialize sets
    let mut sets: Vec<HashSet> = vec![HashSet::new(); max_len];

    // Populate the sets
    for word in &words {
        let len = word.len();
        let mut h: u64 = 0;
        for j in 0..len {
            let c = word.as_bytes()[j] as u64;
            h = (h * base as u64 + c) % MOD;
            if j < max_len {
                sets[j].add(h as i32);
            }
        }
    }

    // Sort and unique each set
    for set in &mut sets {
        set.sort_and_unique();
    }

    // Two-pointer approach
    let mut ans = 0;
    let mut cur_r: usize = 0;
    let mut nxt_r: usize = 0;

    for i in 0..n {
        while nxt_r < n && (nxt_r - i) < max_len {
            if nxt_r + 1 > n {
                break;
            }
            let len = (nxt_r + 1 - i) as usize;
            let pow_val = pow_base[len];
            let hash_val = pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_val % MOD);
            let current_hash = hash_val % MOD;
            let sh = current_hash as i32;
            let prefix_len = nxt_r - i;
            if prefix_len >= max_len {
                break;
            }
            let set = &sets[prefix_len];
            if set.data.binary_search(&sh).is_ok() {
                nxt_r += 1;
            } else {
                break;
            }
        }
        if i == cur_r {
            if i == nxt_r {
                println!("-1");
                return;
            }
            cur_r = nxt_r;
            ans += 1;
        }
    }

    println!("{}", ans);
}