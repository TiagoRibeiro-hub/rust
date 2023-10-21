// * https://adventofcode.com/2015/day/1

use std::fs;

pub fn read_input() -> String {
    fs::read_to_string("/home/tiago/rust/projects/cli/rs_cli/files/floor.txt")
        .expect("Should have been able to read the file")
}
pub fn floor_chars(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => {
                floor += 1;
            }
            ')' => {
                floor -= 1;
            }
            _ => {}
        }
    }
    floor
}

pub fn floor_bytes(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for &item in input.as_bytes().iter() {
        if item == '(' as u8 {
            floor += 1;
        } else if item == ')' as u8 {
            floor -= 1;
        }
    }
    floor
}

pub fn floor_basement_same_var(input: &str) -> i32 {
    let mut floor: i32 = 0;
    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == '(' as u8 {
            floor += 1;
        } else if item == ')' as u8 {
            floor -= 1;
        }
        if floor == -1 {
            floor = i as i32 + 1;
            break;
        }
    }
    floor
}

pub fn floor_basement(input: &str) -> usize {
    let mut floor: i32 = 0;
    let mut res: usize = 0;
    for (i, &item) in input.as_bytes().iter().enumerate() {
        if item == '(' as u8 {
            floor += 1;
        } else if item == ')' as u8 {
            floor -= 1;
        }
        if floor == -1 {
            res = i + 1;
            break;
        }
    }
    res
}

#[test]
fn test() {
    let input = read_input();
    let floor = floor_chars(&input);
    assert_eq!(floor, 138);
    let floor = floor_bytes(&input);
    assert_eq!(floor, 138);
    let floor = floor_basement_same_var(&input);
    assert_eq!(floor, 1771);
    let floor = floor_basement(&input);
    assert_eq!(floor, 1771);
    // println!("{}");
}
