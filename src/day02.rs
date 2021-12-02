#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_report() {
        let input = aoc::vector_from_file::<String>("input/day02/example").unwrap();
        let (horizontal, depth) = travel(&input);
        assert_eq!(horizontal, 15);
        assert_eq!(depth, 10);
    }

    #[test]
    fn example_part_two() {
        let input = aoc::vector_from_file::<String>("input/day02/example").unwrap();
        let (horizontal, _aim, depth) = aim(&input);
        assert_eq!(horizontal, 15);
        assert_eq!(depth, 60);
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    value: i32,
}

impl From<&String> for Command {
    fn from(input: &String) -> Command {
        let mut parts = input.split(' ');
        Command {
            direction: match parts.next() {
                Some("forward") => Direction::Forward,
                Some("up") => Direction::Up,
                Some("down") => Direction::Down,
                _ => Direction::Forward,
            },
            value: i32::from_str_radix(parts.next().unwrap(), 10).unwrap(),
        }
    }
}

fn travel(input: &Vec<String>) -> (i32, i32) {
    input
        .iter()
        .map(|x| {
            let cmd = Command::from(x);
            match cmd.direction {
                Direction::Forward => (cmd.value, 0),
                Direction::Down => (0, cmd.value),
                Direction::Up => (0, -cmd.value),
            }
        })
        .fold((0, 0), |pos, step| (pos.0 + step.0, pos.1 + step.1))
}

fn aim(input: &Vec<String>) -> (i32, i32, i32) {
    // forward, aim, depth
    input
        .iter()
        .map(|x| {
            let cmd = Command::from(x);
            match cmd.direction {
                Direction::Forward => (cmd.value, 0),
                Direction::Down => (0, cmd.value),
                Direction::Up => (0, -cmd.value),
            }
        })
        .fold((0, 0, 0), |state, step| {
            (
                state.0 + step.0,
                state.1 + step.1,
                state.2 + state.1 * step.0,
            )
        })
}

fn main() {
    let input = aoc::vector_from_file::<String>("input/day02/input").unwrap();
    let (horizontal, depth) = travel(&input);
    println!("Horizontal: {}; Depth: {}", horizontal, depth);
    println!("Part 1: {}", horizontal * depth);

    let (horizontal, _aim, depth) = aim(&input);
    println!("Horizontal: {}; Depth: {}", horizontal, depth);
    println!("Part 2: {}", horizontal * depth);
}
