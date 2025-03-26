use std::io;
use std::io::BufRead;
use std::str::SplitWhitespace;

// 将字符串转换为数字
fn str_to_num(str_val: &str) -> i32 {
    let mut num = 0;
    for char_val in str_val.chars() {
        if char_val >= '0' && char_val <= '9' {
            num *= 10;
            num += (char_val as i32 - '0' as i32);
        }
    }
    num
}

// 从id前缀中提取数字部分 (如从"id123"提取123)
fn extract_id_number(id_str: &str) -> i32 {
    // 查找字符串中的数字开始位置
    let mut num_start = None;
    for (index, char_val) in id_str.char_indices() {
        if char_val >= '0' && char_val <= '9' {
            num_start = Some(index);
            break;
        }
    }

    // 如果没有找到数字，返回-1表示错误
    match num_start {
        Some(start_index) => str_to_num(&id_str[start_index..]),
        None => -1,
    }
}

fn count_mentions(number_of_users: i32, events: &Vec<Vec<String>>) -> Vec<i32> {
    let events_size = events.len();
    let mut time_arr: Vec<i32> = Vec::with_capacity(events_size);
    for i in 0..events_size {
        time_arr.push(str_to_num(&events[i][1]));
    }
    let mut order: Vec<usize> = (0..events_size).collect();
    order.sort_by(|a, b| {
        let time_a = time_arr[*a];
        let time_b = time_arr[*b];
        if time_a != time_b {
            time_a.cmp(&time_b)
        } else {
            if events[*a][0] == "O" && events[*b][0] == "M" {
                std::cmp::Ordering::Greater
            } else if events[*a][0] == "M" && events[*b][0] == "O" {
                std::cmp::Ordering::Less
            }
            else {
                std::cmp::Ordering::Equal
            }
        }
    });

    let mut online: Vec<i32> = vec![0; number_of_users as usize];
    let mut mention: Vec<i32> = vec![0; number_of_users as usize];

    for n in 0..events_size {
        let i = order[n];
        if events[i][0] == "M" {
            if events[i][2] == "ALL" {
                for j in 0..number_of_users {
                    mention[j as usize] += 1;
                }
            } else if events[i][2] == "HERE" {
                let time = str_to_num(&events[i][1]);
                for j in 0..number_of_users {
                    if online[j as usize] == 0 {
                        mention[j as usize] += 1;
                    } else if online[j as usize] <= time {
                        online[j as usize] = 0;
                        mention[j as usize] += 1;
                    }
                }
            } else {
                let mut prev = &events[i][2];
                let ids: Vec<&str> = prev.split_whitespace().collect();
                for id_str in ids {
                    let user_id = extract_id_number(id_str);
                    if user_id >= 0 && user_id < number_of_users {
                        mention[user_id as usize] += 1;
                    }
                }
            }
        } else if events[i][0] == "O" {
            let user_id = extract_id_number(&events[i][2]);
            if user_id >= 0 && user_id < number_of_users {
                online[user_id as usize] = str_to_num(&events[i][1]) + 60;
            }
        }
    }
    mention
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().expect("Failed to read line");
    let mut first_line_iter = first_line.split_whitespace();
    let number_of_users: i32 = first_line_iter.next().unwrap().parse().expect("Invalid number of users");
    let events_size: usize = first_line_iter.next().unwrap().parse().expect("Invalid events size");

    let mut events: Vec<Vec<String>> = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let event_line = lines.next().unwrap().expect("Failed to read event line");
        let event_parts: Vec<String> = event_line.split_whitespace().map(|s| s.to_string()).collect();
        events.push(event_parts);
    }

    let result = count_mentions(number_of_users, &events);

    println!("Mentions: {}", result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}