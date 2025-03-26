use std::io;

fn get_largest_outlier(nums: &[i32]) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let mut set = [0; 2001];

    for &num in nums {
        set[(num + 1000) as usize] += 1;
    }

    let mut ans = -1001;
    for &num in nums {
        let curr_sum = total_sum - num;
        if curr_sum % 2 == 0 {
            let half = curr_sum / 2;
            let threshold = if half == num { 1 } else { 0 };

            if half >= -1000 && half <= 1000 {
                if set[(half + 1000) as usize] > threshold {
                    if ans < num {
                        ans = num;
                    }
                }
            }
        }
    }
    ans
}

fn main() {
    // Read the size of the array
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums_size: usize = input.trim().parse().expect("Please type a number!");

    // Read the elements of the array
    let mut nums: Vec<i32> = Vec::with_capacity(nums_size);
    for _ in 0..nums_size {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let num: i32 = input.trim().parse().expect("Please type a number!");
        nums.push(num);
    }

    // Calculate the largest outlier
    let result = get_largest_outlier(&nums);

    // Output the result
    println!("{}", result);
}