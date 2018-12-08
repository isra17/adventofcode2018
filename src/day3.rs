use itertools::Itertools;
use std::collections::HashSet;

pub struct Square {
    id: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Square> {
    input
        .lines()
        .map(|l| {
            let (id, x, y, w, h) = scan_fmt!(l, "#{} @ {},{}: {}x{}", u32, i32, i32, i32, i32);
            Square {
                id: id.unwrap(),
                x: x.unwrap(),
                y: y.unwrap(),
                w: w.unwrap(),
                h: h.unwrap(),
            }
        }).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(squares: &[Square]) -> usize {
    let (_, dup) = squares.iter().fold(
        (HashSet::new(), HashSet::new()),
        |(mut visited, mut dup), square| {
            for (x, y) in iproduct!(square.x..square.x + square.w, square.y..square.y + square.h) {
                if !visited.insert((x, y)) {
                    dup.insert((x, y));
                }
            }
            (visited, dup)
        },
    );
    dup.len()
}

#[aoc(day3, part2)]
pub fn solve_part2(squares: &[Square]) -> u32 {
    *squares
        .iter()
        .tuple_combinations()
        .fold(
            squares.iter().map(|s| s.id).collect::<HashSet<u32>>(),
            |mut candidates, (a, b)| {
                let rabx = ((a.x + a.x + a.w) - (b.x + b.x + b.w)).abs();
                let raby = ((a.y + a.y + a.h) - (b.y + b.y + b.h)).abs();
                let rapbx = (a.x + a.w) - a.x + (b.x + b.w) - b.x;
                let rapby = (a.y + a.h) - a.y + (b.y + b.h) - b.y;
                if rabx < rapbx && raby < rapby {
                    candidates.remove(&a.id);
                    candidates.remove(&b.id);
                };
                candidates
            },
        ).iter()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"
            )),
            4
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"
            )),
            3
        );
    }
}
