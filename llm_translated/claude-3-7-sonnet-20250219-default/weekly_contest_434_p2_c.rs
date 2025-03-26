use std::io::{self, BufRead};

// Convert a string to a number, ignoring non-digit characters
fn str_to_num(s: &str) -> i32 {
    let mut num = 0;
    for c in s.chars() {
        if c.is_digit(10) {
            num *= 10;
            num += c.to_digit(10).unwrap() as i32;
        }
    }
    num
}

// Extract number part from an ID string (e.g., from "id123" extract 123)
fn extract_id_number(id_str: &str) -> i32 {
    let num_start_idx = id_str.chars().position(|c| c.is_digit(10));
    
    match num_start_idx {
        Some(idx) => str_to_num(&id_str[idx..]),
        None => -1, // Return -1 if no digits found
    }
}

fn count_mentions(
    number_of_users: i32,
    events: &Vec<Vec<String>>,
    events_size: i32,
    _events_col_size: &Vec<i32>,
    return_size: &mut i32,
) -> Option<Vec<i32>> {
    let events_size = events_size as usize;
    let number_of_users = number_of_users as usize;
    
    let mut time_arr = vec![0; events_size];
    for i in 0..events_size {
        time_arr[i] = str_to_num(&events[i][1]);
    }
    
    let mut order = (0..events_size).collect::<Vec<usize>>();
    
    // Sort events by time and type
    for i in (1..events_size).rev() {
        for j in 0..i {
            if time_arr[order[j + 1]] < time_arr[order[j]] || 
               (time_arr[order[j + 1]] == time_arr[order[j]] && events[order[j + 1]][0].starts_with("O")) {
                order.swap(j, j + 1);
            }
        }
    }

    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];
    *return_size = number_of_users as i32;

    for n in 0..events_size {
        let i = order[n];
        if events[i][0].starts_with("M") {
            if events[i][2].starts_with("A") {
                // Mention all users
                for j in 0..number_of_users {
                    mention[j] += 1;
                }
            } else if events[i][2].starts_with("H") {
                // Mention hibernating users
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
                // Mention specific users
                let t_ids: Vec<&str> = events[i][2].split_whitespace().collect();
                
                for t_id in t_ids {
                    let user_id = extract_id_number(t_id) as usize;
                    if user_id < number_of_users {
                        mention[user_id] += 1;
                    }
                }
            }
        } else if events[i][0].starts_with("O") {
            // Online event
            let user_id = extract_id_number(&events[i][2]) as usize;
            if user_id < number_of_users {
                online[user_id] = str_to_num(&events[i][1]) + 60;
            }
        }
    }
    
    Some(mention)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read number of users and events
    let first_line = lines.next().unwrap()?;
    let mut iter = first_line.split_whitespace();
    let number_of_users: i32 = iter.next().unwrap().parse().unwrap();
    let events_size: i32 = iter.next().unwrap().parse().unwrap();
    
    // Read events
    let mut events = Vec::with_capacity(events_size as usize);
    let mut events_col_size = vec![3; events_size as usize];
    
    for _ in 0..events_size {
        let mut event = Vec::with_capacity(3);
        
        for _ in 0..3 {
            let item = lines.next().unwrap()?;
            event.push(item);
        }
        
        events.push(event);
    }
    
    let mut return_size = 0;
    match count_mentions(number_of_users, &events, events_size, &events_col_size, &mut return_size) {
        Some(result) => {
            print!("Mentions: ");
            for i in 0..return_size as usize {
                print!("{} ", result[i]);
            }
            println!();
        },
        None => {
            println!("Error: Failed to compute mentions");
        }
    }
    
    Ok(())
}