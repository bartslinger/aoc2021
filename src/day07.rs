#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = aoc::vector_from_comma_separated_file::<i32>("input/day07/example").unwrap();
        let fuel = calculate_minimum_fuel(&input);
        assert_eq!(fuel, 37);
    }
}

fn calculate_minimum_fuel(input: &Vec<i32>) -> u32 {
    // Assuming there is a unique optimal position, this is at a location where at least one crab already is
    input
        .into_iter()
        .map(|position| {
            let fuel: u32 = input.into_iter().map(|x| (position - x).abs() as u32).sum();
            fuel
        })
        .min()
        .unwrap()
}

fn main() {
    let input = aoc::vector_from_comma_separated_file::<i32>("input/day07/input").unwrap();
    let fuel = calculate_minimum_fuel(&input);
    println!("Part 1: {}", fuel);
}
