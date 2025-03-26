use std::io;
use std::vec::Vec;

// 栈的实现
#[derive(Debug)]
struct Stack {
    data: Vec<i32>,
}

impl Stack {
    // 创建栈
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    // 检查栈是否为空
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // 检查栈的大小
    fn size(&self) -> usize {
        self.data.len()
    }

    // 入栈
    fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    // 出栈
    fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }

    // 查看栈顶元素
    fn top(&self) -> Option<&i32> {
        self.data.last()
    }
}

// 计算组合数量的辅助函数
fn count(m: i32, k: i32) -> i64 {
    if m > k {
        ((m as i64 * 2 - k as i64 + 1) * k as i64) / 2
    } else {
        ((m as i64 + 1) * m as i64) / 2
    }
}

// 计算子数组最小值的和
fn sum_subarray_mins(nums: &[i32], k: i32) -> i64 {
    let mut res = 0;
    let mut stack = Stack::new();

    stack.push(-1); // 添加哨兵元素

    for r in 0..nums.len() {
        while stack.size() > 1 && nums[stack.top().unwrap() as usize] >= nums[r] {
            let i = stack.pop().unwrap();
            let l = stack.top().unwrap();
            let cnt = count(r as i32 - l - 1, k) - count(i - l - 1, k) - count(r as i32 - i - 1, k);
            res += nums[i as usize] as i64 * cnt;
        }
        stack.push(r as i32);
    }

    res
}

// 主函数
fn min_max_subarray_sum(nums: &[i32], k: i32) -> i64 {
    // 创建一个临时数组，包含原始数组和一个额外元素
    let mut temp = nums.to_vec();
    temp.push(i32::min_value() / 2); // 添加一个非常小的值作为哨兵

    // 计算子数组最小值的和
    let mut ans = sum_subarray_mins(&temp, k);

    // 将所有元素取反，再次计算
    for i in 0..temp.len() {
        temp[i] = -temp[i];
    }
    temp[temp.len() - 1] = -temp[temp.len() - 1]; // 恢复哨兵元素的符号

    // 从总和中减去取反后的结果
    ans -= sum_subarray_mins(&temp, k);

    ans
}

fn main() -> io::Result<()> {
    // 读取输入
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    let parts: Vec<i32> = input_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = parts[0] as usize;
    let k = parts[1];

    // 分配内存并读取数组
    let mut nums = Vec::with_capacity(n);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line)?;
    nums.extend(
        input_line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
    );

    // 调用函数计算结果
    let result = min_max_subarray_sum(&nums, k);

    // 输出结果
    println!("{}", result);

    Ok(())
}