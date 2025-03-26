use std::io::{self, BufRead};

// Convert a string to a number
fn str_to_num(s: &str) -> i32 {
    let mut num = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            num *= 10;
            num += (c as i32) - ('0' as i32);
        }
    }
    num
}

// Extract the numeric part from an ID string (e.g., extract 123 from "id123")
fn extract_id_number(id_str: &str) -> i32 {
    // Find the position where the numeric part starts
    let num_start = id_str.chars().position(|c| c.is_ascii_digit());
    
    // If no digit is found, return -1 to indicate an error
    match num_start {
        Some(pos) => str_to_num(&id_str[pos..]),
        None => -1,
    }
}

fn count_mentions(number_of_users: i32, events: &Vec<Vec<String>>, events_size: i32) -> Vec<i32> {
    let events_size = events_size as usize;
    let number_of_users = number_of_users as usize;
    
    let mut time_arr = vec![0; events_size];
    for i in 0..events_size {
        time_arr[i] = str_to_num(&events[i][1]);
    }
    
    let mut order: Vec<usize> = (0..events_size).collect();
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

    for n in 0..events_size {
        let i = order[n];
        if events[i][0].starts_with("M") {
            if events[i][2].starts_with("A") {
                for j in 0..number_of_users {
                    mention[j] += 1;
                }
            } else if events[i][2].starts_with("H") {
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
                // Process mentions of specific users
                let parts: Vec<&str> = events[i][2].split_whitespace().collect();
                for part in parts {
                    let user_id = extract_id_number(part);
                    if user_id >= 0 && user_id < number_of_users as i32 {
                        mention[user_id as usize] += 1;
                    }
                }
            }
        } else if events[i][0].starts_with("O") {
            let user_id = extract_id_number(&events[i][2]);
            if user_id >= 0 && user_id < number_of_users as i32 {
                online[user_id as usize] = str_to_num(&events[i][1]) + 60;
            }
        }
    }
    
    mention
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the first line containing numberOfUsers and eventsSize
    let first_line = lines.next().unwrap()?;
    let params: Vec<i32> = first_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    let number_of_users = params[0];
    let events_size = params[1];
    
    // Read events
    let mut events: Vec<Vec<String>> = Vec::with_capacity(events_size as usize);
    for _ in 0..events_size {
        let mut event = Vec::with_capacity(3);
        for _ in 0..3 {
            let item = lines.next().unwrap()?;
            event.push(item);
        }
        events.push(event);
    }
    
    // Calculate mentions
    let result = count_mentions(number_of_users, &events, events_size);
    
    // Print the result
    print!("Mentions: ");
    for i in 0..result.len() {
        print!("{} ", result[i]);
    }
    println!();
    
    Ok(())
}