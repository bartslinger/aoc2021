use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_digits() {
        let displays = parse_input("input/day08/example");
        let count = count_unique_outputs(&displays);
        assert_eq!(count, 26);
    }

    #[test]
    fn test_decode_output() {
        let displays = parse_input("input/day08/example");
        assert_eq!(decode_single_display(&displays[0]), 8394);
        assert_eq!(decode_single_display(&displays[1]), 9781);
        assert_eq!(decode_single_display(&displays[2]), 1197);
    }
}

#[derive(Debug, Clone)]
struct Display {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

fn parse_input(filename: &str) -> Vec<Display> {
    let input: Vec<String> = aoc::vector_from_file(filename).unwrap();
    input
        .into_iter()
        .map(|x| {
            let mut split = x.split('|');
            let patterns: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| {
                    let mut vector: Vec<String> = x.chars().map(|x| x.to_string()).collect();
                    vector.sort();
                    vector.join("")
                })
                .collect();
            assert_eq!(patterns.len(), 10);
            let outputs: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| {
                    let mut vector: Vec<String> = x.chars().map(|x| x.to_string()).collect();
                    vector.sort();
                    vector.join("")
                })
                .collect();
            assert_eq!(outputs.len(), 4);
            Display { patterns, outputs }
        })
        .collect()
}

fn count_unique_outputs(input: &Vec<Display>) -> usize {
    input
        .into_iter()
        .map(|x| {
            let outputs = &x.outputs;
            outputs
                .into_iter()
                .filter(|o| match o.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn pos(input: char) -> usize {
    input as usize - 97
}

fn decode_single_display(input: &Display) -> u32 {
    let s5: Vec<String> = input
        .patterns
        .clone()
        .into_iter()
        .filter(|x| x.len() == 5)
        .collect();
    let s6: Vec<String> = input
        .patterns
        .clone()
        .into_iter()
        .filter(|x| x.len() == 6)
        .collect();
    assert_eq!(s5.len(), 3);
    assert_eq!(s6.len(), 3);

    // Find the missing characters in s6
    let missing3: Vec<char> = s6.into_iter().map(|x| {
        "abcdefg".chars().filter(|c| {
            let has_char = x.contains(*c);
            !has_char
        })
        .nth(0).unwrap()
    }).collect();
    assert_eq!(missing3.len(), 3);
    println!("{:?}", missing3);
    
    // Find middle segment
    let middle_segment = missing3.into_iter()
        .map(|c| {
            // Count how often it occurs in s5
            let count = s5.clone().into_iter().filter(|x| {
                x.contains(c)
            }).count();
            (c, count)
        })
        .filter(|(c, count)| {
            *count == 3
        })
        .map(|x| x.0)
        .nth(0).unwrap();
    println!("Middle segment: {}", middle_segment);
    0
}

fn main() {
    let displays = parse_input("input/day08/input");
    let count = count_unique_outputs(&displays);
    println!("Part 1: {}", count);
}
