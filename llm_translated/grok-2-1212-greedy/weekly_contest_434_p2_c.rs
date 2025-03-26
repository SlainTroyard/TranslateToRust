use std::io::{self, BufRead};

// 将字符串转换为数字
fn str_to_num(s: &str) -> i32 {
    s.chars()
        .filter(|&c| c.is_digit(10))
        .fold(0, |acc, c| acc * 10 + (c as i32 - '0' as i32))
}

// 从id前缀中提取数字部分 (如从"id123"提取123)
fn extract_id_number(id_str: &str) -> i32 {
    let num_start = id_str.find(|c: char| c.is_digit(10)).unwrap_or(id_str.len());
    if num_start == id_str.len() {
        return -1;
    }
    str_to_num(&id_str[num_start..])
}

fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
    let events_size = events.len();
    let mut time_arr: Vec<i32> = events.iter().map(|e| str_to_num(&e[1])).collect();
    let mut order: Vec<usize> = (0..events_size).collect();

    // 排序
    for i in (1..events_size).rev() {
        for j in 0..i {
            if time_arr[order[j + 1]] < time_arr[order[j]] ||
               (time_arr[order[j + 1]] == time_arr[order[j]] && events[order[j + 1]][0] == "O") {
                order.swap(j, j + 1);
            }
        }
    }

    let mut online = vec![0; number_of_users as usize];
    let mut mention = vec![0; number_of_users as usize];

    for &n in &order {
        let i = n;
        if events[i][0] == "M" {
            if events[i][2] == "A" {
                for j in 0..number_of_users as usize {
                    mention[j] += 1;
                }
            } else if events[i][2] == "H" {
                let time = str_to_num(&events[i][1]);
                for j in 0..number_of_users as usize {
                    if online[j] == 0 {
                        mention[j] += 1;
                    } else if online[j] <= time {
                        online[j] = 0;
                        mention[j] += 1;
                    }
                }
            } else {
                let mut prev = &events[i][2];
                while !prev.is_empty() {
                    let (t_id, rest) = if let Some(space_pos) = prev.find(' ') {
                        prev.split_at(space_pos)
                    } else {
                        (prev, "")
                    };
                    let user_id = extract_id_number(t_id);
                    if user_id >= 0 && user_id < number_of_users {
                        mention[user_id as usize] += 1;
                    }
                    prev = rest.trim_start();
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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line: Vec<i32> = lines.next().unwrap()?.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (number_of_users, events_size) = (first_line[0], first_line[1]);

    let mut events = Vec::new();
    for _ in 0..events_size {
        let event: Vec<String> = (0..3).map(|_| lines.next().unwrap().unwrap()).collect();
        events.push(event);
    }

    let result = count_mentions(number_of_users, events);

    print!("Mentions: ");
    for count in result {
        print!("{} ", count);
    }
    println!();

    Ok(())
}