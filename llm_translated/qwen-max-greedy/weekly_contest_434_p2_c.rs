use std::io::{self, BufRead, Write};

// 将字符串转换为数字
fn str_to_num(str: &str) -> i32 {
    let mut num = 0;
    for ch in str.chars() {
        if ch.is_digit(10) {
            num *= 10;
            num += ch.to_digit(10).unwrap() as i32;
        }
    }
    num
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

fn count_mentions(number_of_users: usize, events: &Vec<Vec<String>>, return_size: &mut usize) -> Vec<i32> {
    let mut time_arr = vec![0; events.len()];
    for (i, event) in events.iter().enumerate() {
        time_arr[i] = str_to_num(&event[1]);
    }

    let mut order = (0..events.len()).collect::<Vec<_>>();
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
        match &events[i][0] {
            "M" => {
                match &events[i][2] {
                    "A" => {
                        for j in 0..number_of_users {
                            mention[j] += 1;
                        }
                    },
                    "H" => {
                        let time = str_to_num(&events[i][1]);
                        for j in 0..number_of_users {
                            if online[j] == 0 {
                                mention[j] += 1;
                            } else if online[j] <= time {
                                online[j] = 0;
                                mention[j] += 1;
                            }
                        }
                    },
                    _ => {
                        let ids: Vec<&str> = events[i][2].split_whitespace().collect();
                        for id in ids {
                            let user_id = extract_id_number(id);
                            if user_id >= 0 && (user_id as usize) < number_of_users {
                                mention[user_id as usize] += 1;
                            }
                        }
                    },
                }
            },
            "O" => {
                let user_id = extract_id_number(&events[i][2]);
                if user_id >= 0 && (user_id as usize) < number_of_users {
                    online[user_id as usize] = str_to_num(&events[i][1]) + 60;
                }
            },
            _ => {},
        }
    }
    mention
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let number_of_users: usize = iter.next().unwrap().parse().unwrap();
    let events_size: usize = iter.next().unwrap().parse().unwrap();

    let mut events = vec![vec!["".to_string(); 3]; events_size];
    for i in 0..events_size {
        for j in 0..3 {
            input.clear();
            stdin_lock.read_line(&mut input).unwrap();
            events[i][j] = input.trim().to_string();
        }
    }

    let mut return_size = 0;
    let result = count_mentions(number_of_users, &events, &mut return_size);

    writeln!(stdout_lock, "Mentions: {}", result.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" ")).unwrap();
}