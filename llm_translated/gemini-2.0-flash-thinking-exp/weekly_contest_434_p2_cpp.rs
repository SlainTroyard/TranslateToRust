use std::cmp::Ordering;
use std::io;
use std::io::BufRead;

struct Solution {}

impl Solution {
    fn cmp(a: &Vec<String>, b: &Vec<String>) -> Ordering {
        let time_a: i32 = a[1].parse().unwrap();
        let time_b: i32 = b[1].parse().unwrap();

        if time_a == time_b {
            if a[0] != b[0] {
                if a[0] == "OFFLINE" {
                    Ordering::Less
                } else if b[0] == "OFFLINE" {
                    Ordering::Greater
                } else {
                    Ordering::Equal // Not specified in original code what to do if not OFFLINE
                }
            } else {
                Ordering::Equal
            }
        } else {
            time_a.cmp(&time_b)
        }
    }

    // 提取ID中的数字部分
    fn extract_id(id_str: &str) -> i32 {
        if id_str.starts_with("id") {
            match id_str[2..].parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing ID: {}: {}", id_str, e);
                    0 // 返回默认值
                }
            }
        } else {
            match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Error parsing ID: {}: {}", id_str, e);
                    0 // 返回默认值
                }
            }
        }
    }

    fn update_counts(counts: &mut Vec<i32>, mentions: &str, number_of_users: usize) {
        for id_str in mentions.split_whitespace() {
            match Self::extract_id(id_str) {
                id if id >= 0 && (id as usize) < counts.len() => {
                    counts[id as usize] += 1;
                }
                _ => {
                    eprintln!("Invalid id {}", id_str);
                }
            }
        }
    }

    pub fn count_mentions(
        &self,
        number_of_users: usize,
        events: &mut Vec<Vec<String>>,
    ) -> Vec<i32> {
        events.sort_by(|a, b| Self::cmp(a, b));
        let mut online_times: Vec<i32> = vec![0; number_of_users];
        let mut counts: Vec<i32> = vec![0; number_of_users];
        let mut all_count: i32 = 0;

        for event in events {
            let time: i32 = event[1].parse().unwrap();
            if event[0] == "OFFLINE" {
                match Self::extract_id(&event[2]) {
                    id if id >= 0 && (id as usize) < number_of_users => {
                        online_times[id as usize] = time + 60;
                    }
                    _ => {
                        eprintln!("Invalid id {}", event[2]);
                    }
                }
                continue;
            }

            let mentions = &event[2];
            if mentions == "ALL" {
                all_count += 1;
            } else if mentions == "HERE" {
                for i in 0..number_of_users {
                    if online_times[i] <= time {
                        counts[i] += 1;
                    }
                }
            } else {
                Self::update_counts(&mut counts, mentions, number_of_users);
            }
        }

        if all_count > 0 {
            for count in &mut counts {
                *count += all_count;
            }
        }

        counts
    }
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let first_line = iterator.next().unwrap().unwrap();
    let mut first_line_iter = first_line.split_whitespace();
    let number_of_users: usize = first_line_iter.next().unwrap().parse().unwrap();
    let number_of_events: usize = first_line_iter.next().unwrap().parse().unwrap();

    let mut events: Vec<Vec<String>> = Vec::new();
    for _ in 0..number_of_events {
        let mut event_line = iterator.next().unwrap().unwrap();
        let mut event_parts = event_line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        
        if event_parts[0] == "MESSAGE"{
            if event_parts.len() < 3 {
                let mut second_event_line = iterator.next().unwrap().unwrap();
                let mut second_event_parts = second_event_line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

                if second_event_parts.len() > 0 {

                    if second_event_parts[0] == "ALL" || second_event_parts[0] == "HERE" {
                        event_parts.push(second_event_parts[0].clone());
                    } else {
                        event_parts.push(second_event_parts[0].clone());
                        let mut rest_of_line = String::new();
                        for i in 1..second_event_parts.len(){
                            rest_of_line.push_str(&second_event_parts[i]);
                            if i < second_event_parts.len() - 1{
                                rest_of_line.push_str(" ");
                            }
                        }
                        
                        if !rest_of_line.is_empty(){
                            event_parts.last_mut().unwrap().push_str(" ");
                            event_parts.last_mut().unwrap().push_str(&rest_of_line);
                        }
                        
                    }

                }


            } else if event_parts.len() == 3 {
              //no-op, handled by original code already, just wrong condition here.

            }else{

                let message_type = event_parts[2].clone();
                let mut rest_of_line = String::new();
                if event_parts.len() > 3{
                for i in 3..event_parts.len(){
                        rest_of_line.push_str(&event_parts[i]);
                         if i < event_parts.len() - 1{
                                rest_of_line.push_str(" ");
                            }
                    }
                }
                
                
                event_line = iterator.next().unwrap().unwrap();
                let mut event_parts_new = event_line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();

                if event_parts_new.len() > 0 {
                    if event_parts_new[0] == "ALL" || event_parts_new[0] == "HERE"{

                      event_parts[2] = event_parts_new[0].clone();
                    }
                    else {

                       event_parts[2] = message_type;

                       event_parts[2].push_str(" ");

                       event_parts[2].push_str(&event_parts_new.join(" "));

                    }

                }
            }
        } else if event_parts[0] == "OFFLINE"{
             if event_parts.len() < 3 {
                    let mut second_event_line = iterator.next().unwrap().unwrap();
                    let mut second_event_parts = second_event_line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
                    event_parts.push(second_event_parts[0].clone());
             }

        }

        events.push(event_parts);
    }

    let solution = Solution {};
    let result = solution.count_mentions(number_of_users, &mut events);

    for count in result {
        print!("{} ", count);
    }
    println!();
}