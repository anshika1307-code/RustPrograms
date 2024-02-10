pub fn smallest_ele_q(){
    let arr = [3,2,1,5,6,4];
    let mut smallest = arr[0];
    for i in 1..arr.len(){
        if arr[i] < smallest {
            smallest = arr[i];
        }
    }
    println!("Smallest element in the array is: {}", smallest);
}