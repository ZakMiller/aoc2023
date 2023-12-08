use std::fs;
use std::cmp;

fn get_digit(str: &str, i: usize) -> Option<u32> {
    match (str.chars().nth(i).unwrap()).to_digit(10) {
        None => {},
        Some(d) => return Some(d),
    }
    let end = cmp::min(i + "eight".len(), str.len());
    let cases: Vec<(&str, u32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ];
    let possible_word = &str[i..end];
    for case in cases {
        if possible_word.starts_with(&case.0) {
            return Some(case.1);
        }
    }
    return None;
}

pub fn advent() {
    let contents = fs::read_to_string("input/p1.txt").unwrap();
    let result: u32 = contents.lines().map(|l| {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for (i, _) in l.chars().enumerate() {
            match get_digit(l, i) {
                None => continue,
                Some(d) => {
                    if first == None {
                        first = Some(d);
                    }
                    last = Some(d);
                }
        }
    }
        format!("{}{}", first.unwrap(), last.unwrap()).parse::<u32>().unwrap()
    }).sum();
    println!("{}", result);
}
