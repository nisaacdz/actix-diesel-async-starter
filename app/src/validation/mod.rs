// use regex::Regex;

// #[ser]
// pub struct Phone(String);

// impl Validate for Phone {
//     fn validate(&self) -> Result<(), ValidationErrors> {
//         let re = Regex::new(r"^\+\d{1,3}\d{7,}$").unwrap();
//         if re.is_match(&self.0) {
//             Ok(())
//         } else {
//             let mut errors = ValidationErrors::new();
//             errors.add("phone", ValidationError::new("Invalid phone number format"));
//             Err(errors)
//         }
//     }
// }