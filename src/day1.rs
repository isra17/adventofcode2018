use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|l| {
            l.split(',')
                .map(|x| x.trim().parse())
                .collect::<std::result::Result<Vec<i32>, _>>()
                .unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input
        .iter()
        .cycle()
        .try_fold((0, HashSet::new()), |(freq, mut seen), &x| {
            let new_freq = freq + x;
            if seen.contains(&new_freq) {
                Err(new_freq)
            } else {
                seen.insert(freq);
                Ok((new_freq, seen))
            }
        }).unwrap_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(solve_part1(&input_generator("+1, -2, +3, +1")), 3);
        assert_eq!(solve_part1(&input_generator("+1, +1, +1")), 3);
        assert_eq!(solve_part1(&input_generator("+1, +1, -2")), 0);
        assert_eq!(solve_part1(&input_generator("-1, -2, -3")), -6);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(solve_part2(&input_generator("+1, -2, +3, +1")), 2);
        assert_eq!(solve_part2(&input_generator("+1, -1")), 0);
        assert_eq!(solve_part2(&input_generator("+3, +3, +4, -2, -4")), 10);
        assert_eq!(solve_part2(&input_generator("-6, +3, +8, +5, -6")), 5);
        assert_eq!(solve_part2(&input_generator("+7, +7, -2, -7, -4")), 14);
    }
}
