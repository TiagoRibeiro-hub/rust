
pub fn slice_string(value_str: &str, char: char, before: bool) -> &str {
    let bytes = value_str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == char as u8 {
            if before {
                return &value_str[..i];
            } else {
                return &value_str[i + 1..];
            }
        }
    }
    value_str
}

pub fn get_indexes(str: &str, char: char) -> Vec<usize> {
    let mut idxs: Vec<usize> = Vec::new(); 
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            idxs.push(i);
        }
    }
    idxs
}