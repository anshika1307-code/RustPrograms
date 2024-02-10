pub fn merge2arr_q(){
    let a1 = vec![1, 3, 5, 7, 9];
    let a2 = vec![2, 4, 6, 8, 10];

    let result = merge_arrs(&a1, &a2);

    println!("Merged Sorted Array: {:?} \n", result);
}

fn merge_arrs(a1: &[i32], a2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(a1.len() + a2.len());
    let (mut i, mut j) = (0, 0);

    while i < a1.len() && j < a2.len() {
        if a1[i] <= a2[j] {
            merged.push(a1[i]);
            i += 1;
        } else {
            merged.push(a2[j]);
            j += 1;
        }
    }
    while i < a1.len() {
        merged.push(a1[i]);
        i += 1;
    }

    while j < a2.len() {
        merged.push(a2[j]);
        j += 1;
    }

    merged
}


