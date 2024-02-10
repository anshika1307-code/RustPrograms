pub fn smallest_ele_q(){
    let arr = [3,2,1,5,6,4];
    let k = 4;

    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}th smallest element is: {} \n", k, smallest),
        None => println!("Invalid value of k. \n"),
    }
}

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; 
    }
    let mut sorted = arr.to_vec();
    sorted.sort();

    Some(sorted[k - 1])
}