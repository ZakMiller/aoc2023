use std::fs;
use std::cmp;
fn one() {
    let contents = fs::read_to_string("input/p2.txt").unwrap();
    let lines = contents.split("\n");
    // Game -> grab -> color_count
    let result: i32 = lines.map(|l| {
        let colon_pieces: Vec<&str> = l.split(":").collect();
        let game_number = colon_pieces[0][5..].parse::<i32>().unwrap();
        let grabs: Vec<&str> = colon_pieces[1].trim().split(";").collect();
        
        for grab in grabs {
            for color_count in grab.split(",") {
                let space_separated: Vec<&str> = color_count.trim().split(" ").collect();
                let count = space_separated[0].parse::<i32>().unwrap();
                let color = space_separated[1];
                if color == "red" && count > 12 ||
                    color == "green" && count > 13 ||
                    color == "blue" && count > 14 {
                    return 0;
                }
            }
        }
        return game_number;
    }).sum();
    println!("{}", result);
}

fn two() {
    let contents = fs::read_to_string("input/p2.txt").unwrap();
    let lines = contents.split("\n");
    // Game -> grab -> color_count
    let result: u32 = lines.map(|l| {
        let colon_pieces: Vec<&str> = l.split(":").collect();
        let grabs: Vec<&str> = colon_pieces[1].trim().split(";").collect();
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;
        for grab in grabs {
            for color_count in grab.split(",") {
                let space_separated: Vec<&str> = color_count
                    .trim().split(" ").collect();
                let count = space_separated[0].parse::<u32>().unwrap();
                let color = space_separated[1].trim();
                match color {
                    "red" => max_red = cmp::max(max_red, count),
                    "green" => max_green = cmp::max(max_green, count),
                    "blue" => max_blue = cmp::max(max_blue, count),
                    _ => panic!("invalid color"),
                }
            }
        }
        return max_red * max_green * max_blue;
    }).sum();
    println!("{}", result);
}
pub fn advent() {
    two();
}
