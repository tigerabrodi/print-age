use std::num::ParseIntError;


pub fn parse_age(age: &mut String) -> Result<u32, ParseIntError> {
    age.trim().parse()
}