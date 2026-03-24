// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_create_partitioned_auth_cookie() {
//         let cookie_name = "auth_token";
//         let token_value = "test_token_value_123";

//         let cookie_str = create_partitioned_auth_cookie(cookie_name, token_value.to_string());

//         // Verify the cookie contains all required attributes
//         assert!(
//             cookie_str.contains("auth_token=test_token_value_123"),
//             "Cookie should contain name and value"
//         );
//         assert!(cookie_str.contains("; Secure"), "Cookie should be Secure");
//         assert!(
//             cookie_str.contains("; HttpOnly"),
//             "Cookie should be HttpOnly"
//         );
//         assert!(
//             cookie_str.contains("; SameSite=None"),
//             "Cookie should have SameSite=None"
//         );
//         assert!(cookie_str.contains("; Path=/"), "Cookie should have Path=/");
//         assert!(
//             cookie_str.contains("; Partitioned"),
//             "Cookie should have Partitioned attribute"
//         );

//         // Verify the Partitioned attribute is at the end
//         assert!(
//             cookie_str.ends_with("; Partitioned"),
//             "Partitioned attribute should be appended at the end"
//         );
//     }

//     #[test]
//     fn test_partitioned_cookie_format() {
//         let cookie_str = create_partitioned_auth_cookie("session", "abc123".to_string());

//         // Check that cookie has the expected format
//         let parts: Vec<&str> = cookie_str.split(';').collect();

//         // Should have at least: name=value, Secure, HttpOnly, SameSite=None, Path=/, Partitioned
//         assert!(
//             parts.len() >= 6,
//             "Cookie should have at least 6 parts: {}",
//             cookie_str
//         );

//         // Verify Partitioned is the last attribute
//         let last_part = parts.last().unwrap().trim();
//         assert_eq!(
//             last_part, "Partitioned",
//             "Last attribute should be Partitioned"
//         );
//     }
// }
