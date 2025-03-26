use std::io;
use std::io::Read;

fn cal_time(
    mountain_height: i32,
    worker_times: &[i32],
    target_time: i64,
) -> bool {
    let mut mountain_height = mountain_height as i64;
    for &worker_time in worker_times {
        let mut l_height = 0i64;
        let mut r_height = 1_000_000i64;
        let mut mid_height;
        let mut use_time;

        while r_height >= l_height {
            mid_height = (l_height + r_height) / 2;
            use_time = mid_height * (worker_time as i64 + mid_height * worker_time as i64) / 2;

            if use_time > target_time {
                r_height = mid_height - 1;
            } else {
                l_height = mid_height + 1;
            }
        }
        mountain_height -= r_height;
    }
    mountain_height <= 0
}

fn min_number_of_seconds(mountain_height: i32, worker_times: &[i32]) -> i64 {
    let mut r_time = 1_000_000_000_000_000_000i64;
    let mut l_time = 0i64;
    let mut mid_time;

    while r_time >= l_time {
        mid_time = (r_time + l_time) / 2;
        if cal_time(mountain_height, worker_times, mid_time) {
            r_time = mid_time - 1;
        } else {
            l_time = mid_time + 1;
        }
    }
    r_time + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let mut first_line = lines.next().unwrap().split_whitespace();
    let mountain_height: i32 = first_line.next().unwrap().parse()?;
    let worker_times_size: usize = first_line.next().unwrap().parse()?;

    let mut second_line = lines.next().unwrap().split_whitespace();
    let mut worker_times: Vec<i32> = Vec::new();
    for _ in 0..worker_times_size {
        worker_times.push(second_line.next().unwrap().parse()?);
    }

    println!("{}", min_number_of_seconds(mountain_height, &worker_times));

    Ok(())
}