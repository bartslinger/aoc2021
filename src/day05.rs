use std::collections::HashMap;
use std::ops::{Add, Sub};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        assert_eq!(Vector2D { x: 10, y: -1 }, Vector2D::from_str("10,-1"));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Line2D {
                p1: Vector2D { x: 1, y: 2 },
                p2: Vector2D { x: 10, y: -9 },
            },
            Line2D::from_str("1,2 -> 10,-9")
        );
    }

    #[test]
    fn test_find_overlapping_straight_lines() {
        let input = aoc::vector_from_file("input/day05/example").unwrap();
        let count = find_overlapping_lines(&input, true);
        assert_eq!(count, 5);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vector2D {
    x: i32,
    y: i32,
}
impl Vector2D {
    fn from_str(input: &str) -> Self {
        let mut split = input.split(',');
        let x = i32::from_str_radix(split.next().unwrap(), 10).unwrap();
        let y = i32::from_str_radix(split.next().unwrap(), 10).unwrap();
        Self { x, y }
    }
}

#[derive(Debug, PartialEq)]
struct Line2D {
    p1: Vector2D,
    p2: Vector2D,
}
impl Line2D {
    fn from_str(input: &str) -> Self {
        let mut split = input.split_whitespace();
        let p1 = Vector2D::from_str(split.next().unwrap());
        split.next();
        let p2 = Vector2D::from_str(split.next().unwrap());
        Self { p1, p2 }
    }
}
impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn find_overlapping_lines(input: &Vec<String>, only_straight: bool) -> usize {
    let lines: Vec<Line2D> = input
        .into_iter()
        .map(|x| Line2D::from_str(x.as_str()))
        .collect();
    let mut map = HashMap::<Vector2D, u32>::new();
    for line in lines {
        let diff = line.p2 - line.p1;
        let straight = diff.x == 0 || diff.y == 0;
        if !straight && only_straight {
            continue;
        }
        let step_x = if diff.x != 0 {
            diff.x / diff.x.abs()
        } else {
            0
        };
        let step_y = if diff.y != 0 {
            diff.y / diff.y.abs()
        } else {
            0
        };
        let step = Vector2D {
            x: step_x,
            y: step_y,
        };

        let mut position = line.p1;
        loop {
            match map.get_mut(&position) {
                Some(p) => *p += 1,
                None => {
                    map.insert(position, 1);
                }
            }
            if position == line.p2 {
                break;
            }
            position = position + step;
        }
    }
    let count = map.into_iter().filter(|x| x.1 > 1).count();
    return count;
}

fn main() {
    let input = aoc::vector_from_file("input/day05/input").unwrap();
    let count = find_overlapping_lines(&input, true);
    println!("Part 1: {}", count);

    let count = find_overlapping_lines(&input, false);
    println!("Part 2: {}", count);
}
