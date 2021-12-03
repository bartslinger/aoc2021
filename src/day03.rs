#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_consumption() {
        let input = aoc::vector_from_file::<String>("input/day03/example").unwrap();
        let (gamma, epsilon) = power_consumption(&input);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn test_life_support_rating() {
        let input = aoc::vector_from_file::<String>("input/day03/example").unwrap();
        let oxygen = life_support_rating(&input, '1');
        let co2 = life_support_rating(&input, '0');
        assert_eq!(oxygen, 23);
        assert_eq!(co2, 10);
        assert_eq!(oxygen * co2, 230);
    }
}

fn power_consumption(input: &Vec<String>) -> (u32, u32) {
    let mut bits: Vec<u32> = vec![0; input[0].len()];
    for line in input {
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                bits[i] += 1;
            }
        }
    }

    let half = (input.len() / 2) as u32;
    let (gamma, epsilon) = bits
        .iter()
        .rev()
        .enumerate()
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

fn life_support_rating(input: &Vec<String>, mode: char) -> u32 {
    let mut input = input.clone();
    let binlen = input[0].len();
    for n in 0..binlen {
        // for bit n:
        let (ones, zeros): (Vec<Option<String>>, Vec<Option<String>>) = input
            .iter()
            .map(|x| {
                if x.chars().nth(n) == Some(mode) {
                    (Some(x.clone()), None)
                } else {
                    (None, Some(x.clone()))
                }
            })
            .unzip();
        let ones: Vec<_> = ones.into_iter().filter_map(|x| x).collect();
        let zeros: Vec<_> = zeros.into_iter().filter_map(|x| x).collect();

        if mode == '1' {
            if ones.len() >= zeros.len() {
                input = ones;
            } else {
                input = zeros;
            }
        } else {
            if zeros.len() >= ones.len() {
                input = ones;
            } else {
                input = zeros;
            }
        }
        if input.len() == 1 {
            break;
        }
    }
    u32::from_str_radix(input.get(0).unwrap(), 2).unwrap()
}

fn main() {
    let input = aoc::vector_from_file::<String>("input/day03/input").unwrap();
    let (gamma, epsilon) = power_consumption(&input);
    println!("Gamma: {}, Epsilon: {}", gamma, epsilon);
    println!("Part 1: {}", gamma * epsilon);

    let oxygen = life_support_rating(&input, '1');
    let co2 = life_support_rating(&input, '0');
    println!("Oxygen: {}, CO2: {}", oxygen, co2);
    println!("Part 2: {}", oxygen * co2);
}
