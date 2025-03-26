pub fn beautiful_spli(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut res = 0;
    let mut cnt0 = vec![0; n];
    let mut kmpcnt: i32 = 0;
    cnt0[0] = 0;

    for i in 1..n {
        while kmpcnt > 0 && nums[i] != nums[kmpcnt as usize] {
            kmpcnt = cnt0[(kmpcnt as usize) - 1] as i32;
        }
        if nums[i] == nums[kmpcnt as usize] {
            kmpcnt += 1;
        }
        cnt0[i] = kmpcnt;
        if i % 2 != 0 {
            let len = (i + 1) as i32;
            let numerator = len / 2;
            let denominator = (i + 1 - kmpcnt as usize) as i32;
            if denominator != 0 && numerator % denominator == 0 {
                res += (n - i - 1) as i32;
            }
        }
    }

    for i in 1..n {
        let mut cnt = vec![0; (n - i) as usize];
        let mut end = n;
        let mut kmpcnt: i32 = 0;
        cnt[0] = 0;
        let two_i = 2 * i;
        if two_i < n {
            let idx = two_i - 1;
            let cnt0_val = cnt0[idx];
            let denominator = (two_i as i32) - cnt0_val;
            if denominator != 0 && (i as i32) % denominator == 0 {
                end = std::cmp::min(end, 3 * i);
            }
        }
        for j in (i + 1)..end {
            while kmpcnt > 0 && nums[j] != nums[i + (kmpcnt as usize)] {
                kmpcnt = cnt[(kmpcnt as usize) - 1] as i32;
            }
            if nums[j] == nums[i + (kmpcnt as usize)] {
                kmpcnt += 1;
            }
            let pos = j - i;
            cnt[pos] = kmpcnt;
            let len = (pos + 1) as i32;
            if len % 2 == 0 {
                let divisor = len / 2;
                let denominator = (len - kmpcnt) as i32;
                if denominator != 0 && (divisor % denominator) == 0 {
                    if len as usize == 2 * i {
                        let div = (i - 1) + (len as usize / 2);
                        let cnt0_div = cnt0[div];
                        let denominator2 = (div as i32) + 1 - cnt0_div;
                        if denominator2 != 0 && (i as i32) % denominator2 == 0 {
                            break;
                        }
                    }
                    res += 1;
                }
            }
        }
    }
    res
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = iter.next().expect("No input for n");
    let nums: Vec<i32> = iter.take(n as usize).collect();

    let result = beautiful_spli(&nums);
    println!("{}", result);
}