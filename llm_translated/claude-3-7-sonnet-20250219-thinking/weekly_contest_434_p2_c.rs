use std::io::{self, BufRead};

// Convert string to number, extracting only digits
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

// Extract number from an ID string (e.g., "id123" -> 123)
fn extract_id_number(id_str: &str) -> i32 {
    // Find position of first digit
    let mut num_start = 0;
    for (i, c) in id_str.chars().enumerate() {
        if c.is_digit(10) {
            num_start = i;
            break;
        }
    }
    
    // If no digit found, return -1
    if num_start >= id_str.len() {
        return -1;
    }
    
    str_to_num(&id_str[num_start..])
}

fn count_mentions(
    number_of_users: usize,
    events: &Vec<Vec<String>>,
    events_size: usize,
) -> Vec<i32> {
    // Create array to store event times
    let mut time_arr = vec![0; events_size];
    for i in 0..events_size {
        time_arr[i] = str_to_num(&events[i][1]);
    }
    
    // Create order array for sorting
    let mut order: Vec<usize> = (0..events_size).collect();
    
    // Bubble sort events based on time and type (exactly as in original code)
    for i in (1..events_size).rev() {
        for j in 0..i {
            if time_arr[order[j + 1]] < time_arr[order[j]] || 
               (time_arr[order[j + 1]] == time_arr[order[j]] && events[order[j + 1]][0].starts_with("O")) {
                // Swap
                let t = order[j];
                order[j] = order[j + 1];
                order[j + 1] = t;
            }
        }
    }

    let mut online = vec![0; number_of_users];
    let mut mention = vec![0; number_of_users];
    
    for &idx in &order {
        if events[idx][0].starts_with("M") {
            if events[idx][2].starts_with("A") {
                // Mention All
                for j in 0..number_of_users {
                    mention[j] += 1;
                }
            } else if events[idx][2].starts_with("H") {
                // Mention Here
                let time = str_to_num(&events[idx][1]);
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
                let content = &events[idx][2];
                let mut prev = 0;
                
                for (i, piece) in content.split_whitespace().enumerate() {
                    let user_id = extract_id_number(piece);
                    if user_id >= 0 && user_id < number_of_users as i32 {
                        mention[user_id as usize] += 1;
                    }
                }
            }
        } else if events[idx][0].starts_with("O") {
            // Online event
            let user_id = extract_id_number(&events[idx][2]);
            if user_id >= 0 && user_id < number_of_users as i32 {
                online[user_id as usize] = str_to_num(&events[idx][1]) + 60;
            }
        }
    }
    
    mention
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read number of users and events
    let first_line = lines.next().unwrap()?;
    let mut params = first_line.split_whitespace();
    let number_of_users: usize = params.next().unwrap().parse().unwrap();
    let events_size: usize = params.next().unwrap().parse().unwrap();
    
    // Read events
    let mut events = Vec::with_capacity(events_size);
    for _ in 0..events_size {
        let mut event = Vec::with_capacity(3);
        for _ in 0..3 {
            let item = lines.next().unwrap()?;
            event.push(item);
        }
        events.push(event);
    }
    
    // Process events and get results
    let result = count_mentions(number_of_users, &events, events_size);
    
    // Print results
    print!("Mentions: ");
    for count in &result {
        print!("{} ", count);
    }
    println!();
    
    Ok(())
}