use std::collections::{BTreeSet, HashMap};
use std::io::{self, Read};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ElementCount {
    count: i32,
    value: i32,
}

impl Ord for ElementCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.count, self.value).cmp(&(other.count, other.value))
    }
}

impl PartialOrd for ElementCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct WindowState {
    l: BTreeSet<ElementCount>,
    r: BTreeSet<ElementCount>,
    sum_l: i64,
    cnt: HashMap<i32, i32>,
}

impl WindowState {
    fn new() -> Self {
        WindowState {
            l: BTreeSet::new(),
            r: BTreeSet::new(),
            sum_l: 0,
            cnt: HashMap::new(),
        }
    }

    fn add(&mut self, x: i32) {
        let current_count = *self.cnt.get(&x).unwrap_or(&0);
        if current_count == 0 {
            return;
        }
        let element = ElementCount {
            count: current_count,
            value: x,
        };
        if let Some(first_l) = self.l.first() {
            if element > *first_l {
                self.sum_l += element.count as i64 * element.value as i64;
                self.l.insert(element);
                return;
            }
        }
        self.r.insert(element);
    }

    fn del(&mut self, x: i32) {
        let old_count = *self.cnt.get(&x).unwrap_or(&0);
        if old_count == 0 {
            return;
        }
        let element = ElementCount {
            count: old_count,
            value: x,
        };
        if self.l.remove(&element) {
            self.sum_l -= element.count as i64 * element.value as i64;
        } else {
            self.r.remove(&element);
        }
    }

    fn l_to_r(&mut self) {
        if let Some(element) = self.l.pop_first() {
            self.sum_l -= element.count as i64 * element.value as i64;
            self.r.insert(element);
        }
    }

    fn r_to_l(&mut self) {
        if let Some(element) = self.r.pop_last() {
            self.sum_l += element.count as i64 * element.value as i64;
            self.l.insert(element);
        }
    }
}

fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let k = k as usize;
    let x = x as usize;
    let mut state = WindowState::new();
    let mut ans = vec![0; nums.len().saturating_sub(k) + 1];

    for r in 0..nums.len() {
        let in_num = nums[r];
        state.del(in_num);
        *state.cnt.entry(in_num).or_insert(0) += 1;
        state.add(in_num);

        let l = match (r + 1).checked_sub(k) {
            Some(l) => l,
            None => continue,
        };

        while !state.r.is_empty() && state.l.len() < x {
            state.r_to_l();
        }
        while state.l.len() > x {
            state.l_to_r();
        }

        if l < ans.len() {
            ans[l] = state.sum_l;
        }

        let out_num = nums[l];
        state.del(out_num);
        *state.cnt.entry(out_num).or_insert(0) -= 1;
        state.add(out_num);
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let k: i32 = tokens.next().unwrap().parse().unwrap();
    let x: i32 = tokens.next().unwrap().parse().unwrap();
    let nums_size: usize = tokens.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = tokens
        .by_ref()
        .take(nums_size)
        .map(|s| s.parse().unwrap())
        .collect();

    let ans = find_x_sum(nums, k, x);
    for num in ans {
        print!("{} ", num);
    }
    println!();
}