use std::cmp;
use std::fs;
#[derive(Debug, Clone)]
struct Mapping {
    destination_range_start: u128,
    source_range_start: u128,
    length: u128,
}

#[derive(PartialEq, Debug, Eq)]
struct Range {
    start: u128,
    length: u128,
}
struct LookupStruct {
    list: Vec<Mapping>,
}
trait Lookup {
    fn prepare(&mut self);
    fn lookup(&mut self, i: &Range) -> Vec<Range>;
    fn add(&mut self, m: Mapping);
}

impl Lookup for LookupStruct {
    fn prepare(&mut self) {
        self.list
            .sort_by(|a, b| a.source_range_start.cmp(&b.source_range_start));
    }
    fn lookup(&mut self, i_range: &Range) -> Vec<Range> {
        let mut i = i_range.start;
        let mut ranges_to_return = vec![];
        for (mapping_i, found_mapping) in self.list.clone().iter().enumerate() {
            if i >= found_mapping.source_range_start + found_mapping.length {
                continue;
            }
            if i >= found_mapping.source_range_start
                && i <= found_mapping.source_range_start + found_mapping.length - 1
            {
                let remaining_segment_len =
                    found_mapping.length - (i - found_mapping.source_range_start);
                let remaining_i_range = i_range.length - (i - i_range.start);
                let length = cmp::min(remaining_segment_len, remaining_i_range);
                ranges_to_return.push(Range {
                    start: found_mapping.destination_range_start
                        + (i - found_mapping.source_range_start),
                    length,
                });
                i += length;
                if i > i_range.start + i_range.length - 1 {
                    return ranges_to_return;
                }
            }
            let mut next_mapping = Mapping {
                source_range_start: i,
                destination_range_start: i,
                length: std::u128::MAX,
            }; // Special case if there aren't any more mappings.
            if mapping_i + 1 < self.list.clone().len() - 1 {
                next_mapping = self.list.clone()[mapping_i + 1].clone();
            }
            if i <= next_mapping.source_range_start {
                // If we're here we're between ranges
                let to_next_segment = next_mapping.source_range_start - i;
                let remaining_i_range = i_range.length - (i - i_range.start);
                let length = cmp::min(to_next_segment, remaining_i_range);
                ranges_to_return.push(Range { start: i, length });
                i += length;
                if i > i_range.start + i_range.length - 1 {
                    return ranges_to_return;
                }
            }
        }
        // If we're out of ranges then we finish by matching values.
        ranges_to_return.push(Range {
            start: i,
            length: i_range.length - (i - i_range.start),
        });
        return ranges_to_return;
    }
    fn add(&mut self, m: Mapping) {
        self.list.push(m);
    }
}
fn new_lookup() -> LookupStruct {
    LookupStruct { list: vec![] }
}
fn common(mut seeds: Vec<Range>, lines: Vec<&str>) -> u128 {
    let mut lookup = new_lookup();

    for line in &lines[1..] {
        // Start of new section
        if line.contains(" map:") {
            lookup = new_lookup();
            continue;
        }
        // End of section
        if line.len() == 0 {
            lookup.prepare();
            seeds = seeds.iter().map(|s| lookup.lookup(s)).flatten().collect();
            continue;
        }
        let pieces: Vec<&str> = line.split_whitespace().collect();
        let destination_range_start = pieces[0].parse::<u128>().unwrap();
        let source_range_start = pieces[1].parse::<u128>().unwrap();
        let length = pieces[2].parse::<u128>().unwrap();
        lookup.add(Mapping {
            destination_range_start,
            source_range_start,
            length,
        });
    }

    // Last mapping
    lookup.prepare();
    seeds = seeds
        .iter()
        .map(|s| lookup.lookup(s.to_owned()))
        .flatten()
        .collect();
    seeds
        .into_iter()
        // Can just ignore length, min will always be the start of the range.
        .fold(std::u128::MAX, |minimum, val| cmp::min(minimum, val.start))
}

fn one(contents: String) -> u128 {
    let lines: Vec<&str> = contents.split("\n").collect();
    let seeds: Vec<Range> = lines[0].split(":").collect::<Vec<_>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.parse::<u128>().unwrap())
        .map(|s| Range {
            start: s,
            length: 1,
        })
        .collect();
    return common(seeds, lines);
}

fn two(contents: String) -> u128 {
    let lines: Vec<&str> = contents.split("\n").collect();
    let numbers: Vec<u128> = lines[0].split(":").collect::<Vec<_>>()[1]
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();
    let mut seeds = vec![];
    let mut index = 0;
    while index < numbers.len() {
        let start = numbers[index];
        let length = numbers[index + 1];
        seeds.push(Range { start, length });
        index += 2;
    }
    return common(seeds, lines);
}

pub fn advent() {
    let contents = fs::read_to_string("input/p5.txt").unwrap();
    println!("{}", two(contents));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example() {
        let mut lookup = new_lookup();
        lookup.add(Mapping {
            destination_range_start: 45,
            source_range_start: 77,
            length: 23,
        });
        lookup.add(Mapping {
            destination_range_start: 81,
            source_range_start: 45,
            length: 19,
        });
        lookup.add(Mapping {
            destination_range_start: 68,
            source_range_start: 64,
            length: 13,
        });
        lookup.prepare();
        assert_eq!(
            lookup.lookup(&Range {
                start: 77,
                length: 1
            }),
            vec![Range {
                start: 45,
                length: 1
            }]
        );
    }

    #[test]
    fn test_one() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(one(input.to_string()), 35);
    }
    #[test]
    fn test_two() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(two(input.to_string()), 46);
    }
}
