fn max_good_number(nums: &[i32]) -> i32 {
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;
    let nums_size = nums.len();

    for i in 0..nums_size {
        for j in 0..7 {
            if i == 0 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    nums1 += 1;
                } else {
                    break;
                }
            } else if i == 1 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num2 += 1;
                } else {
                    break;
                }
            } else if i == 2 {
                let s = (nums[i] << j) & 64;
                if s == 0 {
                    num3 += 1;
                } else {
                    break;
                }
            }
        }
    }

    let mut times = [0; 3];
    times[0] = nums1;
    times[1] = num2;
    times[2] = num3;

    let mut store = [0; 3];
    store[1] = 1;
    store[2] = 2;

    for i in 1..=2 {
        for j in 0..=1 {
            let value1 = (nums[store[j] as usize] << (7 - times[store[j + 1] as usize])) + nums[store[j + 1] as usize];
            let value2 = (nums[store[j + 1] as usize] << (7 - times[store[j] as usize])) + nums[store[j] as usize];
            if value2 >= value1 {
                let temp = store[j];
                store[j] = store[j + 1];
                store[j + 1] = temp;
            }
        }
    }

    (nums[store[0] as usize] << (14 - times[store[1] as usize] - times[store[2] as usize])) +
        (nums[store[1] as usize] << (7 - times[store[2] as usize])) + nums[store[2] as usize]
}

fn main() {
    use std::io;

    let mut num_size_str = String::new();
    io::stdin().read_line(&mut num_size_str).expect("Failed to read line");
    let num_size: usize = num_size_str.trim().parse().expect("Invalid input");

    let mut nums = Vec::with_capacity(num_size);
    for _ in 0..num_size {
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("Failed to read line");
        let num: i32 = num_str.trim().parse().expect("Invalid input");
        nums.push(num);
    }

    println!("{}", max_good_number(&nums));
}