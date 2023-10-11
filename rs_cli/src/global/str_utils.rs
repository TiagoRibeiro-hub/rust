
// pub fn slice_string(value_str: &str, char: char, before: bool) -> &str {
//     let bytes = value_str.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == char as u8 {
//             if before {
//                 return &value_str[..i];
//             } else {
//                 return &value_str[i + 1..];
//             }
//         }
//     }
//     value_str
// }