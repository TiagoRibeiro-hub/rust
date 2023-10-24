use std::collections::HashMap;

// https://adventofcode.com/2015/day/3
// Moves are always exactly one house to the north (^), south (v), east (>), or west (<)
pub fn read_input() -> String {
    std::fs::read_to_string("/home/tiago/rust/projects/cli/files/grid_of_houses.txt")
        .expect("Should have been able to read the file")
}
#[allow(unused)]
pub fn delivers_presents(input: &str) -> i32 {
    const NORTH: u8 = 94; // ^
    const SOUTH: u8 = 118; // v
    const EAST: u8 = 60; // <
    const WEST: u8 = 62; // >

    let mut sum: i32 = 1;
    let mut last_y = 0;
    let mut last_x = 0;

    let mut points: HashMap<i32, Vec<i32>> = HashMap::new();
    points.insert(0, vec![last_x]);
 
    for &item in input.as_bytes().iter() {
        if item == NORTH {
            last_y += 1;

            if !points.contains_key(&last_y) {
                points.insert(last_y, vec![last_x]);
                sum+= 1;
            }
            else {
                if let Some(value) = points.get_mut(&last_y) {
                    if !value.contains(&last_x) {
                        value.push(last_x);
                        sum+= 1;
                    }
                };
            }
        } else if item == SOUTH {
            last_y -= 1;

            if !points.contains_key(&last_y) {
                points.insert(last_y, vec![last_x]);
                sum+= 1;
            }
            else {
                if let Some(value) = points.get_mut(&last_y) {
                    if !value.contains(&last_x) {
                        value.push(last_x);
                        sum+= 1;
                    }
                };
            }
        } else if item == EAST {
            last_x -= 1;
            if let Some(value) = points.get_mut(&last_y) {
                if !value.contains(&last_x) {
                    value.push(last_x);
                    sum+= 1;
                }
            };

        } else if item == WEST {
            last_x += 1;
            if let Some(value) = points.get_mut(&last_y) {
                if !value.contains(&last_x) {
                    value.push(last_x);
                    sum+= 1;
                }
            };
        }
    }
    sum
}

#[test]
fn test() {
    let input = read_input();
    //let nr_presents = delivers_presents("v>v<vvv<<vv^v<v>vv>v<<<^^^^^<<^<vv>^>v^>^>^>^>^>"); // 42
    let nr_presents = delivers_presents(&input);
    assert_eq!(nr_presents, 2572);
    println!("{nr_presents}");
}
