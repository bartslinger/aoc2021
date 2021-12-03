
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = aoc::vector_from_file::<String>("input/day03/example").unwrap();
        let (gamma, epsilon) = power_consumption(input);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }
}

fn power_consumption(input: Vec<String>) -> (u32, u32) {
    let mut bits: Vec<u32> = vec![0; input[0].len()];
    for line in &input {
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                bits[i] += 1;
            }
        }
    }

    let half = (input.len() / 2) as u32;
    let (gamma, epsilon) = bits.iter().rev().enumerate()
        .fold((0_u32, 0_u32), |mut f, x| {
            if x.1 > &half {
                f.0 |= 1 << x.0;
            } else {
                f.1 |= 1 << x.0;
            }
            f
        });

    (gamma, epsilon)
}

fn main() {
    let input = aoc::vector_from_file::<String>("input/day03/input").unwrap();
    let (gamma, epsilon) = power_consumption(input);
    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Part 1: {}", gamma * epsilon);
}

