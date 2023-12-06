use std::cmp;
use std::collections::HashSet;
use std::fs;
fn one(contents: String) -> i32 {
    let lines = contents.split("\n");
    let result: i32 = lines
        .map(|l| {
            let sum = points_on_line(l);
            if sum == 0 {
                return 0;
            }
            // 1 -> 1, 2 -> 2, 3 -> 4
            let result = (2 as u32).pow(sum as u32 - 1) as i32;
            return result;
        })
        .sum();
    return result;
}

fn points_on_line(l: &str) -> usize {
    let right_of_colon = l.split(":").collect::<Vec<_>>()[1].trim();

    let winning_numbers = right_of_colon.split("|").collect::<Vec<_>>()[0]
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>();
    let winning_numbers_map: HashSet<&&str> =
        HashSet::from_iter(winning_numbers.iter().clone());

    let mut sum: i32 = 0;
    let my_numbers = right_of_colon.split("|").collect::<Vec<_>>()[1]
        .trim()
        .split_whitespace();
    for num in my_numbers {
        if winning_numbers_map.contains(&num) {
            sum += 1;
        }
    }
    sum as usize
}

fn two(contents: String) -> i32 {
    let lines = contents.split("\n");
    let line_len = lines.clone().collect::<Vec<_>>().len();
    let mut scratchoffs = vec![1; line_len];
    for (i, l) in lines.enumerate() {
        let points = points_on_line(l);
        for j in i + 1..cmp::min(i + points + 1, line_len) {
            scratchoffs[j] += scratchoffs[i];
        }
    }
    return scratchoffs.iter().sum::<usize>() as i32;
}

pub fn advent() {
    let contents = fs::read_to_string("input/p4.txt").unwrap();
    println!("{}", two(contents));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(one(input.to_string()), 13);
    }
    #[test]
    fn test_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(two(input.to_string()), 30);
    }
}
