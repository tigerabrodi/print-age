use std::{io, num::ParseIntError};

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
                 println!("Something went wrong, please make sure have typed a number: {}", error);
                 continue;
             }
         }
    }
}

fn parse_age(age: &mut String) -> Result<u32, ParseIntError> {
    age.trim().parse()
}