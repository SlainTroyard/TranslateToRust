use std::io::{self, BufRead};
use std::cmp::max;

/// Push into the priority queue
fn queue_push(arr: &mut [i64], arr_size: &mut usize, value: i64) {
    let mut son = *arr_size;
    *arr_size += 1;
    let mut father = if son == 0 { -1 } else { ((son - 1) >> 1) as isize };
    
    while father != -1 && value < arr[father as usize] {
        arr[son] = arr[father as usize];
        son = father as usize;
        father = if son == 0 { -1 } else { ((son - 1) >> 1) as isize };
    }
    
    arr[son] = value;
}

/// Pop from the priority queue
fn queue_pop(arr: &mut [i64], arr_size: &mut usize) {
    let mut father = 0;
    *arr_size -= 1;
    
    loop {
        let left = (father << 1) + 1;
        let right = (father << 1) + 2;
        
        if (*arr_size > left && arr[*arr_size] > arr[left])
            || (*arr_size > right && arr[*arr_size] > arr[right]) 
        {
            let son = if *arr_size > right && arr[left] > arr[right] { right } else { left };
            arr[father] = arr[son];
            father = son;
        } else {
            break;
        }
    }
    
    arr[father] = arr[*arr_size];
}

/// High 32 bits of a 64-bit integer
fn high_int(i: i64) -> i32 {
    (i >> 32) as i32
}

/// Low 32 bits of a 64-bit integer
fn low_int(i: i64) -> i32 {
    (i & 0xFFFFFFFF) as i32
}

/// Merge two 32-bit integers into a 64-bit integer
fn mer_long(i: i32, j: i32) -> i64 {
    ((i as i64) << 32) | (j as i64 & 0xFFFFFFFF)
}

/// In a sorted array (without duplicates), find the largest index less than or equal to value.
/// Return -1 if it doesn't exist.
fn binary_search(map: &[i32], value: i32) -> i32 {
    let map_size = map.len();
    if map_size == 0 || value < map[0] {
        return -1;
    }
    
    let mut left = 0;
    let mut right = map_size - 1;
    
    while left < right {
        let mid = (left + right + 1) >> 1;
        if value < map[mid] {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    
    left as i32
}

/// Calculate the highest bit needed for storing values in the binary tree.
/// If the input is 0, record as the first bit from the right.
fn tree_highest_bit(max: i32) -> i32 {
    if max == 0 {
        return 1;
    }
    
    let mut max_val = max;
    let mut i = 1;
    
    while max_val != 0 {
        i += 1;
        max_val >>= 1;
    }
    
    1 << (i - 2)
}

/// Add a value to the binary tree and return the count of values less than or equal to it.
fn tree_push_count(tree_arr: &mut [i32], highest_bit: i32, value: i32) -> i32 {
    let mut i = 0;
    let mut bit = highest_bit;
    let mut result = 0;
    
    // Add the value to the tree while recording the count of values less than it.
    while bit != 0 {
        if (bit & value) != 0 {
            result += tree_arr[((i << 1) + 1) as usize];
            i = (i << 1) + 2;
        } else {
            i = (i << 1) + 1;
        }
        tree_arr[i as usize] += 1;
        bit >>= 1;
    }
    
    // Add the count of the value itself to get the total count of values less than or equal to it.
    result += tree_arr[i as usize];
    result
}

/// Main function to calculate maximum rectangle area
fn max_rectangle_area(x_coord: &[i32], y_coord: &[i32]) -> i64 {
    let n = x_coord.len();
    let tree_size = n * 3;
    
    let mut y_map_size = 0;
    let mut buff_size = 0;
    let mut i = 0;
    let mut k = i32::MIN;
    let mut result: i64 = -1;
    
    let mut x_map = vec![0; n];
    let mut y_map = vec![0; n];
    let mut lists_size = vec![0; n];
    let mut lists_buff = vec![0; n];
    let mut prefix_buff = vec![0; n];
    let mut x_last = vec![-1; n];
    let mut tree_arr = vec![0; tree_size];
    
    let mut lists: Vec<&mut [i32]> = Vec::with_capacity(n);
    let mut prefix: Vec<&mut [i32]> = Vec::with_capacity(n);
    let mut arr2 = vec![0i64; n];
    let mut queue_arr_size = 0;
    
    let highest_bit = tree_highest_bit((n - 1) as i32);
    
    // Discretize all vertical coordinates
    for &y in y_coord {
        queue_push(&mut arr2, &mut queue_arr_size, y as i64);
    }
    
    while queue_arr_size > 0 {
        // Ensure no duplicate values in yMap by comparing with the previous value
        if k < arr2[0] as i32 {
            k = arr2[0] as i32;
            y_map[y_map_size] = k;
            y_map_size += 1;
        }
        queue_pop(&mut arr2, &mut queue_arr_size);
    }
    
    // Enqueue all coordinate points, recording the mapped discretized vertical coordinates
    for j in 0..n {
        let y = binary_search(&y_map[0..y_map_size], y_coord[j]);
        queue_push(&mut arr2, &mut queue_arr_size, mer_long(x_coord[j], y));
    }
    
    // Dequeue column by column
    while queue_arr_size > 0 {
        let mut j = 0;
        
        // Set up the views into the buffers for this column
        lists.push(&mut lists_buff[buff_size..]);
        prefix.push(&mut prefix_buff[buff_size..]);
        
        x_map[i] = high_int(arr2[0]);
        
        // Points with the same horizontal coordinate are treated as column x_map[i]
        while queue_arr_size > 0 && x_map[i] == high_int(arr2[0]) {
            // Record the mapped values of vertical coordinates for this column and the interval prefix sum
            lists[i][j] = low_int(arr2[0]);
            prefix[i][j] = tree_push_count(&mut tree_arr, highest_bit, lists[i][j]);
            
            // Check if it can serve as the top-right corner of a rectangle
            if j > 0 && x_last[lists[i][j] as usize] != -1 && x_last[lists[i][j] as usize] == k {
                // x and y represent the array indices of the two vertices on the left in column xMap[k]
                let k_usize = k as usize;
                let x = binary_search(&lists[k_usize][0..lists_size[k_usize]], lists[i][j]);
                let y = binary_search(&lists[k_usize][0..lists_size[k_usize]], lists[i][j - 1]);
                
                let number = prefix[i][j] - prefix[i][j - 1] - prefix[k_usize][x as usize] + prefix[k_usize][y as usize];
                
                // If x and y are adjacent, and the interval only contains one vertex, the condition is satisfied
                if x - 1 == y && number == 1 {
                    let t = (x_map[i] - x_map[k_usize]) as i64 * (y_map[lists[i][j] as usize] - y_map[lists[i][j - 1] as usize]) as i64;
                    result = max(result, t);
                }
            }
            
            // Update x_last of the current point's vertical coordinate
            k = x_last[lists[i][j] as usize];
            x_last[lists[i][j] as usize] = i as i32;
            
            // Dequeue and increment counter
            queue_pop(&mut arr2, &mut queue_arr_size);
            j += 1;
        }
        
        // Update buffer space and index
        lists_size[i] = j;
        buff_size += j;
        i += 1;
    }
    
    result
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Input the number of points
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut x_coord = vec![0; n];
    let mut y_coord = vec![0; n];
    
    // Input the coordinates of the points
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        x_coord[i] = coords[0];
        y_coord[i] = coords[1];
    }
    
    // Calculate the maximum rectangle area
    let max_area = max_rectangle_area(&x_coord, &y_coord);
    
    // Output the result
    println!("{}", max_area);
}