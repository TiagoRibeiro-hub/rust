use std::fs;

// https://adventofcode.com/2015/day/3
// Moves are always exactly one house to the north (^), south (v), east (>), or west (<)


pub fn read_input() -> String {
    fs::read_to_string("/home/tiago/rust/projects/cli/files/grid_of_houses.txt")
        .expect("Should have been able to read the file")
}

pub fn delivers_presents(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for &item in input.as_bytes().iter() {
        if item == '^' as u8 {
            todo!()
        } else if item == 'v' as u8 {
            todo!()
        } else if item == '>' as u8 {
            todo!()
        } else if item == '<' as u8 {
            todo!()
        }
    }
    sum
}


#[test]
fn test() {
    let input = read_input();
    let nr_presents = delivers_presents(&input);
    // assert_eq!(floor, 1771);
    println!("{nr_presents}");
}