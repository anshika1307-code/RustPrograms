pub fn max_suba_sum_q(){
  let arr = vec![-1, 2, 1, -5, 4];

  let max_sum = max_suba_sum(&arr);

  println!("Maximum Subarray Sum: {}", max_sum);
}

fn max_suba_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut end = arr[0];
    let mut maxi = arr[0];

    for &num in arr.iter().skip(1) {
        end = end.max(num);
        maxi = maxi.max(end);
    }

    maxi
}

