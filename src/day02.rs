#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_report() {
        let input = commands_from_file("input/day02/example");
        let state = travel(&input);
        assert_eq!(state.horizontal, 15);
        assert_eq!(state.depth, 10);
        assert_eq!(answer(state), 150);
    }

    #[test]
    fn example_part_two() {
        let input = commands_from_file("input/day02/example");
        let state = travel_with_aim(&input);
        assert_eq!(state.horizontal, 15);
        assert_eq!(state.depth, 60);
        assert_eq!(answer(state), 900);
    }
}

#[derive(Debug)]
struct Command {
    horizontal: i32,
    down: i32,
}

impl From<&String> for Command {
    fn from(input: &String) -> Command {
        let mut parts = input.split(' ');
        let direction = parts.next().unwrap();
        let value = i32::from_str_radix(parts.next().unwrap(), 10).unwrap();
        match direction {
            "forward" => Command {
                horizontal: value,
                down: 0,
            },
            "up" => Command {
                horizontal: 0,
                down: -value,
            },
            "down" => Command {
                horizontal: 0,
                down: value,
            },
            _ => panic!("Incorrect input"),
        }
    }
}

fn commands_from_file(filename: &str) -> Vec<Command> {
    let input = aoc::vector_from_file::<String>(filename).unwrap();
    input
        .iter()
        .map(|x| Command::from(x))
        .collect()
}

#[derive(Default)]
struct TravelState {
    horizontal: i32,
    depth: i32,
}

fn travel(input: &Vec<Command>) -> TravelState {
    input
        .iter()
        .fold(TravelState::default(), |mut state, step| {
            state.horizontal += step.horizontal;
            state.depth += step.down;
            state
        })
}

fn travel_with_aim(input: &Vec<Command>) -> TravelState {
    // forward, aim, depth
    input
        .iter()
        .fold((TravelState::default(), 0), |mut state, step| {
            state.0.horizontal += step.horizontal;
            state.0.depth += state.1 * step.horizontal;
            state.1 += step.down;
            state
        })
        .0
}

fn answer(state: TravelState) -> i32 {
    state.horizontal * state.depth
}

fn main() {
    let input = commands_from_file("input/day02/input");
    let state = travel(&input);
    println!("Horizontal: {}; Depth: {}", state.horizontal, state.depth);
    println!("Part 1: {}", answer(state));

    let state = travel_with_aim(&input);
    println!("Horizontal: {}; Depth: {}", state.horizontal, state.depth);
    println!("Part 2: {}", answer(state));
}
