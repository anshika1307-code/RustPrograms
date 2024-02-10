pub fn reverse_q(){
  let str = "Hello, Good Morning!";
  let rev_str = rev_str(str);

  println!("Original string: {}", str);
  println!("Reversed string: {}", rev_str);
}

fn rev_str(input: &str) -> String {
    let mut rev = String::new();

    for i in input.chars().rev() {
        rev.push(i);
    }

    rev
}