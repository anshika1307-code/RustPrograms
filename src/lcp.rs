
pub fn lcp_q(){
      let strs = vec!["where", "what", "when"];

      let c_pre = longest_common_prefix(&strs);

      if c_pre.is_empty() {
          println!("No common prefix found.");
      } else {
          println!("The longest common prefix is: '{}'", c_pre);
      }
}

fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_str = strs[0];

    for (index, char) in first_str.chars().enumerate() {
        if !strs.iter().all(|s| s.chars().nth(index) == Some(char)) {
            return first_str[..index].to_string();
        }
    }

    first_str.to_string()
}


