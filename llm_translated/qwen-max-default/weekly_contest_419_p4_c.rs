use std::cmp::Ordering;
use std::io::{self, BufRead};

// 频率和值对
#[derive(Clone, Copy)]
struct Pair {
    val: i32,   // 元素值
    freq: i32,  // 频率
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.freq, self.val) == (other.freq, other.val)
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.freq.cmp(&self.freq) {
            Ordering::Equal => other.val.cmp(&self.val),
            other => other,
        }
    }
}

fn find_x_sum(nums: &[i32], k: usize, x: usize) -> Vec<i64> {
    let nums_size = nums.len();
    let return_size = nums_size - k + 1;
    let mut result = vec![0; return_size];

    const TABLE_SIZE: usize = 1031;  // 质数，减少冲突
    const MAX_CHAIN: usize = 32;     // 链表最大长度

    let mut hash_table: Vec<Vec<Pair>> = vec![vec![Pair { val: 0, freq: 0 }; MAX_CHAIN]; TABLE_SIZE];
    let mut active_elements: Vec<Pair> = Vec::with_capacity(k);
    let mut active_count = 0;

    // 处理第一个窗口
    for &val in nums.iter().take(k) {
        let hash = (val as usize) % TABLE_SIZE;
        let mut found = false;

        // 查找链表中是否存在
        for pair in &mut hash_table[hash] {
            if pair.freq == 0 {
                break; // 到达链表末尾
            }
            if pair.val == val {
                pair.freq += 1;
                found = true;
                break;
            }
        }

        // 如果未找到，添加新元素
        if !found {
            for pair in &mut hash_table[hash] {
                if pair.freq == 0 {
                    *pair = Pair { val, freq: 1 };
                    break;
                }
            }
        }
    }

    // 收集所有活跃元素
    for row in &hash_table {
        for pair in row {
            if pair.freq > 0 {
                active_elements.push(*pair);
                active_count += 1;
                if active_count >= k {
                    break;
                }
            }
        }
        if active_count >= k {
            break;
        }
    }

    // 对活跃元素排序
    active_elements.sort();

    // 计算第一个窗口的结果
    let count_to_sum = active_count.min(x);
    result[0] = active_elements.iter().take(count_to_sum).map(|p| (p.val as i64) * (p.freq as i64)).sum();

    // 滑动窗口
    for i in 1..=nums_size - k {
        let out_val = nums[i - 1];  // 移出窗口的元素
        let in_val = nums[i + k - 1]; // 移入窗口的元素

        // 更新哈希表 - 减少移出元素的频率
        let out_hash = (out_val as usize) % TABLE_SIZE;
        for pair in &mut hash_table[out_hash] {
            if pair.val == out_val {
                pair.freq -= 1;
                break;
            }
        }

        // 更新哈希表 - 增加移入元素的频率
        let in_hash = (in_val as usize) % TABLE_SIZE;
        let mut found = false;
        for pair in &mut hash_table[in_hash] {
            if pair.val == in_val {
                pair.freq += 1;
                found = true;
                break;
            }
        }

        // 如果是新元素，添加它
        if !found {
            for pair in &mut hash_table[in_hash] {
                if pair.freq == 0 {
                    *pair = Pair { val: in_val, freq: 1 };
                    break;
                }
            }
        }

        // 重置活跃元素计数
        active_elements.clear();
        active_count = 0;

        // 重新收集所有活跃元素
        for row in &hash_table {
            for pair in row {
                if pair.freq > 0 {
                    active_elements.push(*pair);
                    active_count += 1;
                    if active_count >= k {
                        break;
                    }
                }
            }
            if active_count >= k {
                break;
            }
        }

        // 对活跃元素排序
        active_elements.sort();

        // 计算当前窗口的结果
        let count_to_sum = active_count.min(x);
        result[i] = active_elements.iter().take(count_to_sum).map(|p| (p.val as i64) * (p.freq as i64)).sum();
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let line = lines.next().unwrap().unwrap();
    let nums_size: usize = line.parse().unwrap();

    let line = lines.next().unwrap().unwrap();
    let nums: Vec<i32> = line.split_whitespace()
                             .map(|s| s.parse().unwrap())
                             .collect();

    let result = find_x_sum(&nums, k, x);

    for sum in result {
        print!("{} ", sum);
    }
    println!();
}