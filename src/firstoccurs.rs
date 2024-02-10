pub fn firstoccurs_q() {
    let nums = vec![1,11, 2, 2, 2, 3, 3, 9, 40, 50];
    let target = 3;

    match find_index(&nums, target) {
        Some(index) => println!("Target value found at index: {}", index),
        None => println!("Target value not found in the array."),
    }
}

fn find_index(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    return None
}