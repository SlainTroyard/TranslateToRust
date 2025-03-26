use std::io::{self, BufRead, Write};
use std::str::FromStr;

// 将字符串转换为数字
fn str_to_num(str: &str) -> i32 {
    str.chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit as i32)
}

// 从id前缀中提取数字部分 (如从"id123"提取123)
fn extract_id_number(id_str: &str) -> i32 {
    // 查找字符串中的数字开始位置
    if let Some(num_start) = id_str.find(|c: char| c.is_digit(10)) {
        str_to_num(&id_str[num_start..])
    } else {
        -1
    }
}

fn count_mentions(number_of_users: usize, events: &Vec<Vec<String>>, return_size: &mut usize) -> Option<Vec<i32>> {
    let mut time_arr = vec![0; events.len()];
    for (i, event) in events.iter().enumerate() {
        time_arr[i] = str_to_num(&event[1]);
    }

    let mut order = (0..events.len()).collect::<Vec<usize>>();
    for i in (0..events.len()).rev() {
        for j in 0..i {
            if time_arr[order[j + 1]] < time_arr[order[j]] || (time_arr[order[j + 1]] == time_arr[order[j]] && events[order[j + 1]][0].starts_with('O')) {
                order.swap(j, j + 1);
            }
        }
    }

    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];
    *return_size = number_of_users;

    for n in 0..events.len() {
        let i = order[n];
        if events[i][0].starts_with('M') {
            if events[i][2].starts_with('A') {
                for j in 0..number_of_users {
                    mention[j] += 1;
                }
            } else if events[i][2].starts_with('H') {
                let time = str_to_num(&events[i][1]);
                for j in 0..number_of_users {
                    if online[j] == 0 {
                        mention[j] += 1;
                    } else if online[j] <= time {
                        online[j] = 0;
                        mention[j] += 1;
                    }
                }
            } else {
                let t_ids: Vec<&str> = events[i][2].split_whitespace().collect();
                for t_id in t_ids {
                    let user_id = extract_id_number(t_id);
                    if user_id >= 0 && (user_id as usize) < number_of_users {
                        mention[user_id as usize] += 1;
                    }
                }
            }
        } else if events[i][0].starts_with('O') {
            let user_id = extract_id_number(&events[i][2]);
            if user_id >= 0 && (user_id as usize) < number_of_users {
                online[user_id as usize] = str_to_num(&events[i][1]) + 60;
            }
        }
    }
    Some(mention)
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    // 读取用户数和事件数
    stdin.read_line(&mut buffer).unwrap();
    let mut iter = buffer.trim().split_whitespace();
    let number_of_users = usize::from_str(iter.next().unwrap()).unwrap();
    let events_size = usize::from_str(iter.next().unwrap()).unwrap();

    // 读取事件
    let mut events = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let event: Vec<String> = buffer
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        events.push(event);
    }

    let mut return_size = 0;
    if let Some(result) = count_mentions(number_of_users, &events, &mut return_size) {
        writeln!(stdout, "Mentions: ").unwrap();
        for i in 0..return_size {
            write!(stdout, "{} ", result[i]).unwrap();
        }
        writeln!(stdout).unwrap();
    } else {
        writeln!(stdout, "Error: Failed to compute mentions").unwrap();
    }
}