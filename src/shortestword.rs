pub fn shortestword_q(){
     let str = "This is a rust code";

     if let Some(shortest) = shortest_word(str) {
         println!("The shortest word is: '{}' \n", shortest);
     } else {
         println!("No words found in the string. \n");
     }
  
  }
  
  
fn shortest_word(str: &str) -> Option<&str> {
    str.split_whitespace().min_by_key(|i| i.len())
}



