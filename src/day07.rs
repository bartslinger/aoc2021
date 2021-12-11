#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_fuel() {
        let input = aoc::vector_from_comma_separated_file::<i32>("input/day07/example").unwrap();
        let fuel = calculate_minimum_fuel_constant_rate(&input);
        assert_eq!(fuel, 37);
    }

    #[test]
    fn test_fuel_cost() {
        assert_eq!(fuel_cost(1), 1);
        assert_eq!(fuel_cost(2), 3);
        assert_eq!(fuel_cost(3), 6);
        assert_eq!(fuel_cost(11), 66);
    }

    #[test]
    fn test_minimum_fuel() {
        let input = aoc::vector_from_comma_separated_file::<i32>("input/day07/example").unwrap();
        let fuel = calculate_minimum_fuel(&input);
        assert_eq!(fuel, 168);
    }
}

fn calculate_minimum_fuel_constant_rate(input: &Vec<i32>) -> u32 {
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

fn fuel_cost(distance: u32) -> u32 {
    ((1 + distance) * distance) / 2
}

fn calculate_minimum_fuel(input: &Vec<i32>) -> u32 {
    let start = *input.into_iter().min().unwrap();
    let end = *input.into_iter().max().unwrap();
    let range = start..end;
    let total_minimum_fuel = range
        .map(|position| {
            let fuel: u32 = input
                .into_iter()
                .map(|x| {
                    let distance = (position - x).abs() as u32;
                    fuel_cost(distance)
                })
                .sum();
            fuel
        })
        .min()
        .unwrap();
    total_minimum_fuel
}

fn main() {
    let input = aoc::vector_from_comma_separated_file::<i32>("input/day07/input").unwrap();
    let fuel = calculate_minimum_fuel_constant_rate(&input);
    println!("Part 1: {}", fuel);

    let fuel = calculate_minimum_fuel(&input);
    println!("Part 2: {}", fuel);
}
