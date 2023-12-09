use std::collections::HashMap;
use std::fs;

fn two(content: &str) -> i32 {
    let results = content.lines().map(|s| {
        let mut sequence: Vec<i32> = s.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        let mut first_in_each = vec![sequence.first().unwrap().clone()];
        while !sequence.iter().all(|n| *n == 0) {
            let mut new_sequence = vec![];
            for i in 1..sequence.len() {
                new_sequence.push(sequence[i] - sequence[i-1]);
            }
            first_in_each.push(new_sequence.first().unwrap().clone());
            sequence = new_sequence;
        }
        // I'm sure there's a better way of doing this...
        let mut new_end = 0;
        for element in first_in_each.iter().rev() {
            new_end = element - new_end;
        }
        return new_end;
    });
    return results.sum();
}

fn one(content: &str) -> i32 {
    let results = content.lines().map(|s| {
        let mut sequence: Vec<i32> = s.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        let mut last_in_each = vec![sequence.last().unwrap().clone()];
        while !sequence.iter().all(|n| *n == 0) {
            let mut new_sequence = vec![];
            for i in 1..sequence.len() {
                new_sequence.push(sequence[i] - sequence[i-1]);
            }
            last_in_each.push(new_sequence.last().unwrap().clone());
            sequence = new_sequence;
        }
        last_in_each.iter().sum::<i32>()
    });
    return results.sum();
}

pub fn advent() {
    let contents = fs::read_to_string("input/p9.txt").unwrap();
    let result = two(&contents);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(one(input), 114);
    }

    #[test]
    fn test_two() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(two(input), 2);
    }
}
