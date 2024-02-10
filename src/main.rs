use std::io;
mod palidrome;
mod merge2arr;
mod reverse;
mod max_suba_sum;
mod max_depth;
mod smallest_ele;
mod array_median;
mod firstoccurs;
mod lcp;
mod prime;
mod shortestword;

use palidrome::palidrome_q;
use merge2arr::merge2arr_q;
use reverse::reverse_q;
use max_suba_sum::max_suba_sum_q;
use max_depth::max_depth_q;
use smallest_ele::smallest_ele_q;
use array_median::array_median_q;
use firstoccurs::firstoccurs_q;
use lcp::lcp_q;
use prime::prime_q;
use shortestword::shortestword_q;

fn main() {
  loop{
     println!("Enter the question number to see its output:\n 1 -> Palindrome \n 2 -> First Occurrence \n 3 -> Shortest Word \n 4 -> Prime or not \n 5 -> Median of Array \n 6 -> longest common prefix \n 7 -> Kth Smallest Element \n 8 -> Max Depth of a tree \n 9 -> Reverse a string \n 10 -> Check if a number is prime \n 11 -> Merge 2 Arrays \n 12 -> Max Subarray Sum\n 0 -> Exit");

        let mut ques = String::new();
        io::stdin()
            .read_line(&mut ques)
            .expect("Failed to read line");
        let number: i32 = match ques.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid question 
    number.");
                return;
            }
        };
        match number {
          1 => palidrome_q(),
          2 => firstoccurs_q(),
          3 => shortestword_q(),
          4 | 10 => prime_q(),
          5 => array_median_q(),
          6 => lcp_q(),
          7 => smallest_ele_q(),
          8 => max_depth_q(),
          9 => reverse_q(),

          11 => merge2arr_q(),
          12 => max_suba_sum_q(),
          0 => {
              println!("Exiting.... Goodbye!");
              break;
          }
            _ => println!("It's something else!"),
          
        }
  }
   
    
}