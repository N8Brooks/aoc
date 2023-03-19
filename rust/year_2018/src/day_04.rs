use std::collections::HashMap;

use chrono::NaiveDateTime;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Event {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

fn parse_input(input: &str) -> HashMap<usize, [usize; 60]> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (?P<event>falls asleep|Guard #(?P<guard_id>\d+) begins shift|wakes up)$").unwrap();
    }
    RE.captures_iter(input)
        .map(|caps| {
            let time = NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M")
                .unwrap()
                .timestamp()
                / 60;
            let event = {
                let event_name = caps.name("event").unwrap().as_str();
                let guard_id = caps.name("guard_id");
                match (event_name, guard_id) {
                    ("falls asleep", None) => Event::FallAsleep,
                    ("wakes up", None) => Event::WakeUp,
                    (_, Some(guard_id)) => Event::BeginShift(guard_id.as_str().parse().unwrap()),
                    _ => panic!("unkown event"),
                }
            };
            (time, event)
        })
        .sorted_unstable_by_key(|(time, _)| *time)
        .scan(None, |guard_id, (time, event)| {
            *guard_id = match event {
                Event::BeginShift(guard_id) => Some(guard_id),
                _ => *guard_id,
            };
            Some((
                usize::try_from(time.rem_euclid(60)).unwrap(),
                guard_id.expect("guard began shift"),
                event,
            ))
        })
        .tuple_windows()
        .filter(|((_, _, event), _)| *event == Event::FallAsleep)
        .map(|((minute_0, guard_id_0, _), (minute_1, _, _))| (guard_id_0, minute_0..minute_1))
        .into_grouping_map_by(|(guard_id, _)| *guard_id)
        .fold([0; 60], |mut acc, _, (_, minutes)| {
            for minute in minutes {
                acc[minute] += 1;
            }
            acc
        })
}

pub fn part_1(input: &str) -> usize {
    let (guard_id, minute_counts) = parse_input(input)
        .into_iter()
        .max_by_key(|(_, counts)| counts.iter().sum::<usize>())
        .unwrap();
    let minute = (0..60).max_by_key(|minute| minute_counts[*minute]).unwrap();
    guard_id * minute
}

pub fn part_2(input: &str) -> usize {
    let (guard_id, minute, _) = parse_input(input)
        .into_iter()
        .map(|(guard_id, minute_counts)| {
            let (minute, count) = minute_counts
                .into_iter()
                .enumerate()
                .max_by_key(|(_, count)| *count)
                .unwrap();
            (guard_id, minute, count)
        })
        .max_by_key(|(_, _, count)| *count)
        .unwrap();
    guard_id * minute
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const EXAMPLE: &str = "[1518-11-01 00:00] Guard #10 begins shift
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
[1518-11-05 00:55] wakes up";

    const INPUT: &str = include_str!("../../../testdata/year_2018/day_04.txt");

    #[test_case(EXAMPLE, 240; "example")]
    #[test_case(INPUT, 94040; "input")]
    fn part_1(input: &str, expected: usize) {
        assert_eq!(super::part_1(input), expected);
    }

    #[test_case(EXAMPLE, 4455; "example")]
    #[test_case(INPUT, 39940; "input")]
    fn part_2(input: &str, expected: usize) {
        assert_eq!(super::part_2(input), expected);
    }
}
