use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let (two, three) = input
        .lines()
        .map(|l| {
            l.chars()
                .fold(HashMap::new(), |mut acc, c| {
                    acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
                    acc
                }).values()
                .fold((0, 0), |(two, three), count| match count {
                    2 => (two | 1, three),
                    3 => (two, three | 1),
                    _ => (two, three),
                })
        }).fold((0, 0), |(twos, threes), (two, three)| {
            (twos + two, three + threes)
        });
    return two * three;
}
fn quick_cmp(a: &str, b: &str) -> Option<usize> {
    a.chars()
        .zip(b.chars())
        .enumerate()
        .try_fold(None, |found, (i, (ca, cb))| match (ca == cb, found) {
            (true, _) => Some(found),
            (false, None) => Some(Some(i)),
            (false, Some(_)) => None,
        }).and_then(|o| o)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    lines
        .iter()
        .enumerate()
        .find_map(|(i, s1)| {
            lines
                .iter()
                .skip(i + 1)
                .find_map(|s2| quick_cmp(s1, s2).map(|i| (s1, i)))
        }).map(|(s, i)| {
            let mut s = String::from(*s);
            s.remove(i);
            s
        }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab"),
            12
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            solve_part2(
                "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"
            ),
            "fgij"
        );
    }
}
