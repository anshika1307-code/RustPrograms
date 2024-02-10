pub fn array_median_q(){
    let sort_arr = vec![1, 2, 3, 4, 5];
    let median_arr = median(&sort_arr);
    println!("Median of given sorted array : {}", median_arr);
}

fn median(sort_arr: &[i32]) -> f64 {
    let len = sort_arr.len();

    if len % 2 == 0 {
        // len is even
        let mid1 = sort_arr[len / 2 - 1];
        let mid2 = sort_arr[len / 2];
        return (mid1 + mid2) as f64 / 2.0;
    } else {
        // len is odd
        return sort_arr[len / 2] as f64;
    }
}


