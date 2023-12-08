use std::collections::HashMap;
use std::fs;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{start} = ({left}, {right})")]
struct Instruction {
  start: String,
  left: String,
  right: String,
}

#[derive(Debug)]
struct ZPrediction {
    indices: Vec<usize>,
    len: usize,
}
fn get_end(initial_location: String, m: &HashMap<String, Instruction>, pattern: &str) -> usize {
    let mut location = initial_location.clone();
    let mut jumps = 0;
    let mut pattern_index = 0;
    while location != initial_location || jumps == 0 {
        let instruction = m.get(&location).unwrap();
        match pattern.chars().nth(pattern_index).unwrap() {
            'L' => location = instruction.left.clone(),
            'R' => location = instruction.right.clone(),
             n => panic!("unexpected {}", n),
        }
        jumps += 1;
        pattern_index += 1;
        if pattern_index == pattern.len() {
            pattern_index = 0;
        }
        if location.contains("Z") {
            return jumps;
        }
    }
    panic!("expected to end");
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}


fn get_lcm_pair(a: usize, b: usize) -> usize {
    (a / gcd(a,b)) * b
}

fn get_lcm(v: Vec<usize>) -> usize {
    let mut lcm = *v.first().unwrap();
    for element in v {
        lcm = get_lcm_pair(lcm, element);
    }
    lcm
}

fn two_smart(content: &str) -> usize {
    let lines: Vec<&str> = content.lines().collect();
    let pattern = lines[0];

    let instructions = lines.iter().skip(2).map(|l| l.parse::<Instruction>().unwrap());
    let mut m: HashMap<String, Instruction> = HashMap::new();
    for instruction in instructions.clone() {
        m.insert(instruction.start.clone(), instruction);
    }
    let locations: Vec<String> = instructions
        .clone().filter(|i| i.start.contains("A")).map(|i| i.start).collect();
    let z_ends: Vec<usize> = locations.iter().map(|l| get_end(l.clone(), &m, &pattern)).collect();
    return get_lcm(z_ends);
}

fn two(content: &str) -> u32 {
    let lines: Vec<&str> = content.lines().collect();
    let pattern = lines[0];

    let instructions = lines.iter().skip(2).map(|l| l.parse::<Instruction>().unwrap());
    let mut m: HashMap<String, Instruction> = HashMap::new();
    for instruction in instructions.clone() {
        m.insert(instruction.start.clone(), instruction);
    }
    let mut locations: Vec<String> = instructions
        .clone().filter(|i| i.start.contains("A")).map(|i| i.start).collect();
    let mut jumps = 0;
    let mut pattern_index = 0;
    while !locations.iter().all(|l| l.contains("Z")) {
        let matching = locations.iter().filter(|l| l.contains("Z")).count();
        if  matching > 0 {
            println!("{} - {}", jumps, matching);
        }
        for (i,location) in locations.clone().iter().enumerate() {
            let instruction = m.get(location).unwrap();
            match pattern.chars().nth(pattern_index).unwrap() {
                'L' => locations[i] = instruction.left.clone(),
                'R' => locations[i] = instruction.right.clone(),
                n => panic!("unexpected {}", n),
            }
        }
        jumps += 1;
        pattern_index += 1;
        if pattern_index == pattern.len() {
            pattern_index = 0;
        }

    }
    return jumps;
}
fn one(content: &str) -> u32 {
    let lines: Vec<&str> = content.lines().collect();
    let pattern = lines[0];

    let instructions = lines.iter().skip(2).map(|l| l.parse::<Instruction>().unwrap());
    let mut m: HashMap<String, Instruction> = HashMap::new();
    for instruction in instructions {
        m.insert(instruction.start.clone(), instruction);
    }
    let mut location = "AAA";
    let mut jumps = 0;
    let mut pattern_index = 0;
    while location != "ZZZ" {
        let instruction = m.get(location).unwrap();
        match pattern.chars().nth(pattern_index).unwrap() {
            'L' => location = instruction.left.as_ref(),
            'R' => location = instruction.right.as_ref(),
             n => panic!("unexpected {}", n),
        }
        jumps += 1;
        pattern_index += 1;
        if pattern_index == pattern.len() {
            pattern_index = 0;
        }

    }
    return jumps;
}

pub fn advent() {
    let contents = fs::read_to_string("input/p8.txt").unwrap();
    let result = two_smart(&contents);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_one() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(one(input), 6);
    }

    #[test]
    fn test_two_smart() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(two_smart(input), 6);
    }
    #[test]
    fn test_two() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(two(input), 6);
    }
}
