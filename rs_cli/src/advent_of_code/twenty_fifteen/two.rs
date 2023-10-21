use std::{cmp::min, fs::File, io};

// https://adventofcode.com/2015/day/2

fn read_input() -> Vec<String> {
    std::fs::read_to_string("/home/tiago/rust/projects/cli/files/wrapper_paper.txt")
        .expect("Should have been able to read the file")
        .lines()
        .map(String::from)
        .collect()
}
fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    super::read_lines("/home/tiago/rust/projects/cli/files/wrapper_paper.txt")
}

fn papper_wrapper_needed(l: u32, w: u32, h: u32) -> u32 {
    let one = l * w;
    let two = w * h;
    let three = h * l;
    (2 * one + 2 * two + 2 * three) + min(min(one, two), three)
}

fn get_indexes(str: &str, char: char) -> Vec<usize> {
    let mut idxs: Vec<usize> = Vec::new();
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            idxs.push(i);
        }
    }
    idxs
}

pub fn wrapper_paper_read_input() -> u32 {
    let cubes = read_input();
    let mut sum: u32 = 0;
    for value in cubes.iter() {
        let indxs = get_indexes(value, 'x');
        let l = value[..indxs[0]].parse::<u32>().unwrap();
        let w = value[indxs[0] + 1..indxs[1]].parse::<u32>().unwrap();
        let h = value[indxs[1] + 1..].parse::<u32>().unwrap();
        sum += papper_wrapper_needed(l, w, h);
    }
    sum
}

pub fn wrapper_paper_read_lines() -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(value) = line {
                let indxs = get_indexes(&value, 'x');
                let l = value[..indxs[0]].parse::<u32>().unwrap();
                let w = value[indxs[0] + 1..indxs[1]].parse::<u32>().unwrap();
                let h = value[indxs[1] + 1..].parse::<u32>().unwrap();
                sum += papper_wrapper_needed(l, w, h);
            }
        }
    }
    sum
}

fn get_side_values(str: &str, char: char) -> (u32, u32, u32) {
    let mut sides: (u32, u32, u32) = (0, 0, 0);
    let mut last: usize = 0;
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            if last == 0 {
                last = i + 1;
                sides.0 = str[..i].parse::<u32>().unwrap();
            } else {
                sides.1 = str[last..i].parse::<u32>().unwrap();
                sides.2 = str[i + 1..].parse::<u32>().unwrap();
                break;
            }
        }
    }
    sides
}

pub fn wrapper_paper_read_input_get_side_values() -> u32 {
    let cubes = read_input();
    let mut sum: u32 = 0;
    for value in cubes.iter() {
        let sides = get_side_values(value, 'x');
        sum += papper_wrapper_needed(sides.0, sides.1, sides.2);
    }
    sum
}

pub fn wrapper_paper_read_lines_get_side_values() -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(value) = line {
                let sides = get_side_values(&value, 'x');
                sum += papper_wrapper_needed(sides.0, sides.1, sides.2);
            }
        }
    }
    sum
}

fn area(str: &str, char: char) -> u32 {
    let mut sides: (u32, u32, u32) = (0, 0, 0);
    let mut last: usize = 0;
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            if last == 0 {
                last = i + 1;
                sides.0 = str[..i].parse::<u32>().unwrap();
            } else {
                sides.1 = str[last..i].parse::<u32>().unwrap();
                sides.2 = str[i + 1..].parse::<u32>().unwrap();
                break;
            }
        }
    }
    papper_wrapper_needed(sides.0, sides.1, sides.2)
}

pub fn wrapper_paper_read_input_area() -> u32 {
    let cubes = read_input();
    let mut sum: u32 = 0;
    for value in cubes.iter() {
        sum += area(value, 'x');
    }
    sum
}

pub fn wrapper_paper_read_lines_area() -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(value) = line {
                sum += area(&value, 'x');
            }
        }
    }
    sum
}

fn area_sum(str: &str, char: char, sum: &mut u32) {
    let mut sides: (u32, u32, u32) = (0, 0, 0);
    let mut last: usize = 0;
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            if last == 0 {
                last = i + 1;
                sides.0 = str[..i].parse::<u32>().unwrap();
            } else {
                sides.1 = str[last..i].parse::<u32>().unwrap();
                sides.2 = str[i + 1..].parse::<u32>().unwrap();
                break;
            }
        }
    }
    *sum += papper_wrapper_needed(sides.0, sides.1, sides.2)
}

pub fn wrapper_paper_read_input_area_sum() -> u32 {
    let cubes = read_input();
    let mut sum: u32 = 0;
    for value in cubes.iter() {
        area_sum(value, 'x', &mut sum);
    }
    sum
}

pub fn wrapper_paper_read_lines_area_sum() -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(value) = line {
                area_sum(&value, 'x', &mut sum);
            }
        }
    }
    sum
}

fn ribbon_total(str: &str, char: char) -> u32 {
    let mut sides: (u32, u32, u32) = (0, 0, 0);
    let mut last: usize = 0;
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            if last == 0 {
                last = i + 1;
                sides.0 = str[..i].parse::<u32>().unwrap();
            } else {
                sides.1 = str[last..i].parse::<u32>().unwrap();
                sides.2 = str[i + 1..].parse::<u32>().unwrap();
                break;
            }
        }
    }
    let mut sorted: [u32; 3] = [sides.0, sides.1, sides.2];
    sorted.sort_unstable();
    (2 * sorted[0] + 2 * sorted[1]) + (sides.0 * sides.1 * sides.2)
}

fn ribbon_total_2(str: &str, char: char) -> u32 {
    let mut sorted: [u32; 3] = [0, 0, 0];
    let mut last: usize = 0;
    let mut bow: u32 = 0;
    for (i, &item) in str.as_bytes().iter().enumerate() {
        if item == char as u8 {
            if last == 0 {
                last = i + 1;
                bow = str[..i].parse::<u32>().unwrap();
                sorted[0] = bow;
            } else {
                let a = str[last..i].parse::<u32>().unwrap();
                let b = str[i + 1..].parse::<u32>().unwrap();
                sorted[1] = a;
                sorted[2] = b;
                bow = bow * a * b;
                break;
            }
        }
    }

    sorted.sort_unstable();
    (2 * sorted[0] + 2 * sorted[1]) + bow
}

pub fn ribbon_length() -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(value) = line {
                sum += ribbon_total(&value, 'x');
            }
        }
    }
    sum
}

pub fn ribbon_length_2() -> u32 {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(value) = line {
                sum += ribbon_total_2(&value, 'x');
            }
        }
    }
    sum
}

#[test]
fn test() {
    let papper = wrapper_paper_read_lines_area();
    assert_eq!(papper, 1588178);
    let ribbon = ribbon_length_2();
    assert_eq!(ribbon, 3783758);
    println!("{ribbon}");
}
