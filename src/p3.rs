use std::collections::HashMap;
use std::fs;
fn p3_one(contents: String) -> u32 {
    let lines = contents.split("\n");
    let adjacent_space_map = get_adjacent_spaces_map(lines.clone().collect());
    let numbers = get_numbers(lines.collect());

    return numbers
        .into_iter()
        .filter(
            |n| match n.points.iter().find(|p| adjacent_space_map.contains_key(p)) {
                None => false,
                _ => true,
            },
        )
        .map(|n| n.value)
        .sum();
}

fn get_adjacent_spaces_map(lines: Vec<&str>) -> HashMap<Point, bool> {
    let mut map = HashMap::new();
    for (li, line) in lines.iter().enumerate() {
        for (ci, char) in line.chars().enumerate() {
            if char.is_digit(10) || char == '.' {
                continue;
            }
            let cases: Vec<(i32, i32)> = vec![
                (1, 1),
                (1, 0),
                (1, -1),
                (0, 1),
                (0, -1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ];
            for case in cases {
                let x = (ci as i32 + case.0) as i32;
                let y = (li as i32 + case.1) as i32;
                if x < 0 || y < 0 {
                    continue;
                }
                let p = Point { x, y };
                map.insert(p, true);
            }
        }
    }
    return map;
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Number {
    value: u32,
    points: Vec<Point>,
}

fn new_number(nums: String, li: usize, start_c: usize) -> Number {
    let value = format!("{}", nums).parse::<u32>().unwrap();
    let mut points: Vec<Point> = vec![];
    for x in start_c..start_c + nums.len() {
        points.push(Point {
            x: x as i32,
            y: li as i32,
        });
    }
    return Number { value, points };
}

fn get_numbers(lines: Vec<&str>) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    for (li, line) in lines.iter().enumerate() {
        let mut number_characters: String = "".to_string();
        for (ci, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number_characters.push(char);
            }
            if !char.is_digit(10) || ci == line.len() - 1 {
                if !number_characters.is_empty() {
                    let mut start = ci - (number_characters.len() - 1);
                    if !char.is_digit(10) {
                        start -= 1;
                    }
                    numbers.push(new_number(number_characters, li, start));
                    number_characters = "".to_string();
                }
            }
        }
    }
    return numbers;
}

fn get_gear_points(lines: Vec<&str>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];
    for (li, line) in lines.iter().enumerate() {
        for (ci, char) in line.chars().enumerate() {
            if char == '*' {
                points.push(Point {
                    x: ci as i32,
                    y: li as i32,
                });
            }
        }
    }
    return points;
}

fn p3_two(contents: String) -> u32 {
    let lines = contents.split("\n");
    let numbers = get_numbers(lines.clone().collect());

    let mut coord_to_number_map = HashMap::new();
    for number in numbers.clone() {
        for point in number.clone().points {
            coord_to_number_map.insert(point, number.clone());
        }
    }
    let gear_points = get_gear_points(lines.collect());

    return gear_points
        .into_iter()
        .map(|g| {
            let cases: Vec<(i32, i32)> = vec![
                (1, 1),
                (1, 0),
                (1, -1),
                (0, 1),
                (0, -1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
            ];
            let mut adjacent_numbers = HashMap::new();
            for case in cases {
                let x = (g.x as i32 + case.0);
                let y = (g.y as i32 + case.1);
                if x < 0 || y < 0 {
                    continue;
                }
                let p = Point { x, y };
                match coord_to_number_map.get(&p) {
                    Some(v) => {
                        // Won't work if a gear is adjacent to two numbers
                        // with the same value. Using an ID instead would work.
                        adjacent_numbers.insert(v.value, v);
                    }
                    None => {}
                }
            }
            if adjacent_numbers.len() != 2 {
                return 0;
            }
            return adjacent_numbers
                .values()
                .fold(1, |product, number| product * number.value);
        })
        .sum();
}

pub fn advent() {
    let contents = fs::read_to_string("input/p3.txt").unwrap();
    let result = p3_two(contents);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_all() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(p3_one(input.to_string()), 4361);
    }

    #[test]
    fn test_allp2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(p3_two(input.to_string()), 467835);
    }
    #[test]
    fn test_get_adjacent_spaces_map() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let lines = input.split("\n");
        let m = get_adjacent_spaces_map(lines.collect());
        for (k, _) in m {
            println!("{:?}", k);
        }
    }

    #[test]
    fn test_get_numbers() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let lines = input.split("\n");
        let numbers = get_numbers(lines.collect());
        for number in numbers {
            println!("{:?}", number);
        }
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        //        assert_eq!(bad_add(1, 2), 3);
    }
}
