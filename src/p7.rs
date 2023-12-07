use std::collections::HashMap;
use std::fs;

fn common(mut lines: Vec<Line>, jokers: bool) -> u32 {
    lines.sort_by(|a, b| {
        if a.hand_type != b.hand_type {
            return a.hand_type.cmp(&b.hand_type);
        }
        let mut j_val = 11;
        if jokers {
            j_val = 0;
        }

        let lookup = HashMap::from([('A', 14), ('K', 13), ('Q', 12), ('J', j_val), ('T', 10)]);
        for i in 0..5 {
            let a_char = a.hand.chars().nth(i).unwrap();
            let b_char = b.hand.chars().nth(i).unwrap();

            // second unwrap_or is because Rust is greedy and will evaluate
            // either way
            let a_val: u32 = lookup
                .get(&a_char)
                .unwrap_or(&a_char.to_digit(10).unwrap_or(0))
                .to_owned();
            let b_val: u32 = lookup
                .get(&b_char)
                .unwrap_or(&b_char.to_digit(10).unwrap_or(0))
                .to_owned();
            if a_val != b_val {
                return a_val.cmp(&b_val);
            }
        }
        // Same
        return a.hand_type.cmp(&b.hand_type);
    });
    let mut total = 0;
    for (i, line) in lines.iter().collect::<Vec<_>>().iter().enumerate() {
        // sorted asc
        total += line.bid * (i + 1) as u32;
    }
    return total;
}

struct Line {
    hand: String,
    bid: u32,
    hand_type: u32,
}

fn get_hand_type_jokers(hand: &str) -> u32 {
    let joker_count = hand.chars().filter(|c| *c == 'J').collect::<Vec<_>>().len();
    if joker_count == 5 {
        return 7;
    }
    let mut map: HashMap<char, u32> = HashMap::new();
    for char in hand.chars().filter(|c| *c != 'J') {
        match map.get(&char) {
            Some(count) => map.insert(char, count + 1),
            None => map.insert(char, 1),
        };
    }
    let mut values: Vec<u32> = map.values().cloned().collect();
    values.sort_by(|a, b| b.cmp(a));
    values[0] += joker_count as u32;
    let lookup_key = values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    match lookup_key.as_ref() {
        "5" => 7,
        "4,1" => 6,
        "3,2" => 5,
        "3,1,1" => 4,
        "2,2,1" => 3,
        "2,1,1,1" => 2,
        "1,1,1,1,1" => 1,
        _ => panic!("unexpected result {}", lookup_key),
    }
}
fn get_hand_type(hand: &str) -> u32 {
    let mut map: HashMap<char, u32> = HashMap::new();
    for char in hand.chars() {
        match map.get(&char) {
            Some(count) => map.insert(char, count + 1),
            None => map.insert(char, 1),
        };
    }
    let mut values: Vec<u32> = map.values().cloned().collect();
    values.sort_by(|a, b| b.cmp(a));
    let lookup_key = values
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    match lookup_key.as_ref() {
        "5" => 7,
        "4,1" => 6,
        "3,2" => 5,
        "3,1,1" => 4,
        "2,2,1" => 3,
        "2,1,1,1" => 2,
        "1,1,1,1,1" => 1,
        _ => panic!("unexpected result {}", lookup_key),
    }
}

fn one(content: &str) -> u32 {
    let lines: Vec<Line> = content
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|l| {
            let pieces = l.split(" ").collect::<Vec<_>>();
            return Line {
                hand: pieces[0].to_string(),
                bid: pieces[1].parse::<u32>().unwrap(),
                hand_type: get_hand_type(pieces[0]),
            };
        })
        .collect();
    return common(lines, false);
}
fn two(content: &str) -> u32 {
    let lines: Vec<Line> = content
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|l| {
            let pieces = l.split(" ").collect::<Vec<_>>();
            return Line {
                hand: pieces[0].to_string(),
                bid: pieces[1].parse::<u32>().unwrap(),
                hand_type: get_hand_type_jokers(pieces[0]),
            };
        })
        .collect();
    return common(lines, true);
}
pub fn advent() {
    let contents = fs::read_to_string("input/p7.txt").unwrap();
    let result = two(&contents);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(one(input), 6440);
    }

    #[test]
    fn test_two() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(two(input), 5905);
    }
}
