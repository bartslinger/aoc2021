#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_report() {
        let input = aoc::vector_from_file::<u32>("input/day01/example").unwrap();
        let output = count_increments(&input);
        assert_eq!(output, 7);
    }
    
    #[test]
    fn example_part_two() {
        let input = aoc::vector_from_file::<u32>("input/day01/example").unwrap();
        let output = count_sliding_window_increments(&input);
        assert_eq!(output, 5);
    }
}

pub fn count_increments(input: &Vec<u32>) -> u32 {
    input.as_slice()
        .windows(2)
        .map(|w| (w[1] > w[0]) as u32)
        .sum()
}

pub fn count_sliding_window_increments(input: &Vec<u32>) -> u32 {
    let window_sums: Vec<u32> = input.as_slice()
        .windows(3)
        .map(|w| w.iter().sum())
        .collect();
    count_increments(&window_sums)
}

fn main() {
    let input = aoc::vector_from_file::<u32>("input/day01/input").unwrap();
    let output = count_increments(&input);
    println!("Part 1: {}", output);
    
    let output = count_sliding_window_increments(&input);
    println!("Part 2: {}", output);
}