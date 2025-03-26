use std::io;
use std::io::Read;

fn max_good_number(nums: &[i32]) -> i32 {
    let nums_size = nums.len();
    let mut nums1 = 0;
    let mut num2 = 0;
    let mut num3 = 0;

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

    let mut times = vec![0; 3];
    times[0] = nums1;
    times[1] = num2;
    times[2] = num3;

    let mut store = vec![0, 1, 2];

    for i in 1..=2 {
        for j in 0..=1 {
            let value1 = (nums[store[j] as usize] << (7 - times[store[j + 1] as usize])) +
                         nums[store[j + 1] as usize];
            let value2 =
                (nums[store[j + 1] as usize] << (7 - times[store[j] as usize])) + nums[store[j] as usize];
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let num_size: usize = lines.next().unwrap().parse().unwrap();

    let mut nums: Vec<i32> = Vec::new();
    for _ in 0..num_size {
        let num: i32 = lines.next().unwrap().parse().unwrap();
        nums.push(num);
    }

    println!("{}", max_good_number(&nums));

    Ok(())
}