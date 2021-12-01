#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_report() {
        let input = aoc::vector_from_file::<u32>("input/day01/example").unwrap();
        let output = count_increments(input);
        assert_eq!(output, 7);
    }
}

pub fn count_increments(input: Vec<u32>) -> u32 {
    input.as_slice()
        .windows(2)
        .map(|w| (w[1] > w[0]) as u32)
        .sum()
}    

fn main() {
    let input = aoc::vector_from_file::<u32>("input/day01/input1").unwrap();
    let output = count_increments(input);
    println!("Part 1: {}", output);
}