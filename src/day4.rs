use chrono::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

type Schedule = HashMap<Date<Utc>, Vec<u32>>;
type Schedules = HashMap<u32, Schedule>;

pub enum Action {
    Guard(u32),
    FallAsleep,
    WakeUp,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Box<Schedules> {
    Box::new(
        input
            .lines()
            .map(|l| {
                let (date, action) = scan_fmt!(l, "[{[0-9 -:]}] {/.*/}", String, String);
                let action = action.unwrap();
                let date = Utc
                    .datetime_from_str(&date.unwrap(), "%Y-%m-%d %H:%M")
                    .unwrap();
                let action = match action.chars().next().unwrap() {
                    'G' => {
                        let id = scan_fmt!(&action, "Guard #{} ", u32);
                        Action::Guard(id.unwrap())
                    }
                    'w' => Action::WakeUp,
                    'f' => Action::FallAsleep,
                    _ => panic!(),
                };
                (date, action)
            }).sorted_by(|(d1, _), (d2, _)| Ord::cmp(d1, d2))
            .iter()
            .fold(
                (0, Schedules::new()),
                |(guard, mut schedules), (datetime, action)| {
                    if let Action::Guard(new_guard) = action {
                        return (*new_guard, schedules);
                    }

                    {
                        let minute = datetime.minute() as usize;
                        let schedule = schedules.entry(guard).or_insert(Schedule::new());
                        let day = schedule.entry(datetime.date()).or_insert(Vec::new());
                        match action {
                            Action::FallAsleep => day.resize(minute, 0),
                            Action::WakeUp => day.resize(minute, 1),
                            _ => panic!(),
                        };
                    }

                    (guard, schedules)
                },
            ).1,
    )
}

#[aoc(day4, part1)]
pub fn solve_part1(schedules: &Schedules) -> u32 {
    let (guard, schedule) = schedules
        .iter()
        .max_by_key(|(_, schedule)| {
            schedule.iter().fold(0 as u32, |sum, (_, minutes)| {
                sum + minutes.iter().sum::<u32>()
            })
        }).unwrap();

    let (minute, _) = schedule
        .iter()
        .fold(vec![0; 60], |mut acc, (_, minutes)| {
            acc.iter_mut().zip(minutes).for_each(|(a, m)| *a += m);
            acc
        }).iter()
        .enumerate()
        .max_by_key(|(_, x)| *x)
        .unwrap();

    guard * minute as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(schedules: &Schedules) -> u32 {
    let (guard, minute, _) = schedules
        .iter()
        .map(|(guard, schedule)| {
            let minutes = schedule.iter().fold(vec![0; 60], |mut acc, (_, minutes)| {
                acc.iter_mut().zip(minutes).for_each(|(a, m)| *a += m);
                acc
            });
            let (minute, count) = minutes.iter().enumerate().max_by_key(|(_, x)| *x).unwrap();
            (guard, minute, *count)
        }).max_by_key(|(_, _, c)| *c)
        .unwrap();
    guard * minute as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(
            solve_part1(&input_generator(
                "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            )),
            240
        );
    }

    fn examples_part2() {
        assert_eq!(
            solve_part2(&input_generator(
                "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"
            )),
            4455
        );
    }
}
