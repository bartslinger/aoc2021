#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input: Vec<u32> = aoc::vector_from_comma_separated_file("input/day06/example").unwrap();
        let school = school_from_vec(&input);
        let size = school_size(school, 80);

        assert_eq!(size, 5934);
    }
}

type LanternFishSchool = [usize; 9];
fn school_from_vec(input: &Vec<u32>) -> LanternFishSchool {
    let mut school: LanternFishSchool = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish in input {
        school[*fish as usize] += 1;
    }
    school
}

fn step(prev: LanternFishSchool) -> LanternFishSchool {
    let mut next: LanternFishSchool = [
        prev[1],           // 0
        prev[2],           // 1
        prev[3],           // 2
        prev[4],           // 3
        prev[5],           // 4
        prev[6],           // 5
        prev[0] + prev[7], // 6
        prev[8],           // 7
        prev[0],           // 8
    ];
    next
}

fn school_size(mut school: LanternFishSchool, days: usize) -> usize {
    for day in 1..=days {
        school = step(school);
    }
    school.into_iter().sum()
}

fn main() {
    let input: Vec<u32> = aoc::vector_from_comma_separated_file("input/day06/input").unwrap();
    let school = school_from_vec(&input);
    let size = school_size(school, 80);
    println!("Part 1: {}", size);
}
