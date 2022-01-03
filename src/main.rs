use std::{io};
use age::parse_age;

fn main() {
    loop {
     println!("Please enter your age");

     
         let mut age = String::new();
     
         io::stdin().read_line(&mut age).expect("Failed to read line!");

         let age = parse_age(&mut age);
     
         match age {
             Ok(age) => {
                 println!("Your age is {}", age);
                 break;
             },
             Err(error) => {
                 println!("Something went wrong: {}", error);
                 continue;
             }
         }
    }
}