use std::num::ParseIntError;


pub fn parse_age(age: &mut String) -> Result<u32, ParseIntError> {
    age.trim().parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_age() {
        let mut age = String::from("2");

        assert_eq!(Ok(2), parse_age(&mut age))
    }
}