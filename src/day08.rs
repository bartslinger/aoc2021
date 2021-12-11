#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_unique_digits() {
        let displays = parse_input("input/day08/example");
        let count = count_unique_outputs(displays);
        assert_eq!(count, 26);
    }
}

#[derive(Debug)]
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
                .map(|x| String::from(x))
                .collect();
            assert_eq!(patterns.len(), 10);
            let outputs: Vec<String> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| String::from(x))
                .collect();
            assert_eq!(outputs.len(), 4);
            Display { patterns, outputs }
        })
        .collect()
}

fn count_unique_outputs(input: Vec<Display>) -> usize {
    input
        .into_iter()
        .map(|x| {
            x.outputs
                .into_iter()
                .filter(|o| match o.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn main() {
    let displays = parse_input("input/day08/input");
    let count = count_unique_outputs(displays);
    println!("Part 1: {}", count);
}
