pub fn palidrome_q() {

    let str1 = "Able Elba";
    if is_palindrome(str1) {
        println!("'{}' is a palindrome!",str1);
    } else {
        println!("'{}' is not a palindrome.",str1);
    }
}

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase(); 
    let reversed: String = s.chars().rev().collect();
    s == reversed
}