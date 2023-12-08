use std::fs;

fn common(pairs: Vec<(u128, u128)>) -> u128 {
    let result: u128 = pairs
        .iter()
        .map(|p| {
            let time = p.0;
            let distance = p.1;
            // exhaustive
            let mut smallest_winning_charging_time: i128 = -1;
            let mut smallest_losing_charging_time_after_winning: i128 = -1;
            for charging_time in 1..time {
                let our_distance = charging_time * (time - charging_time);
                if our_distance > distance && smallest_winning_charging_time == -1 {
                    smallest_winning_charging_time = charging_time as i128;
                }
                if our_distance <= distance
                    && smallest_winning_charging_time != -1
                    && smallest_losing_charging_time_after_winning == -1
                {
                    smallest_losing_charging_time_after_winning = charging_time as i128;
                    return (smallest_losing_charging_time_after_winning
                        - smallest_winning_charging_time) as u128;
                }
            }

            panic!("no solution found");
        })
        .product();
    return result;
}
fn one(content: &str) -> u128 {
    let lines: Vec<&str> = content.split("\n").collect();
    let times = lines[0]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap());
    let distance = lines[1]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap());
    let pairs: Vec<(u128, u128)> = times.into_iter().zip(distance.into_iter()).collect();
    return common(pairs);
}
fn two(content: &str) -> u128 {
    let lines: Vec<&str> = content.split("\n").collect();
    let time = lines[0]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u128>()
        .unwrap();
    let distance = lines[1]
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u128>()
        .unwrap();
    let pairs = vec![(time, distance)];
    return common(pairs);
}
pub fn advent() {
    let contents = fs::read_to_string("input/p6.txt").unwrap();
    let result = two(&contents);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(one(input), 288);
    }

    #[test]
    fn test_two() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(two(input), 71503);
    }
}
