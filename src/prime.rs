pub fn prime_q(){
   let num = 13;

   if is_prime(num) {
       println!("{} is a prime number. \n", num);
   } else {
       println!("{} is not a prime number. \n", num);
   }
}

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; 
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false; 
        }
    }

    true 
}
