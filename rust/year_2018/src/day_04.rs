use std::collections::HashMap;

use chrono::NaiveDateTime;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
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
            let time = {
                NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M")
                    .unwrap()
                    .and_utc()
                    .timestamp()
                    / 60
            };
            let event = {
                let event_name = caps.name("event").unwrap().as_str();
                let guard_id = caps.name("guard_id");
                match (event_name, guard_id) {
                    ("falls asleep", None) => Event::FallAsleep,
                    ("wakes up", None) => Event::WakeUp,
                    (_, Some(guard_id)) => Event::BeginShift(guard_id.as_str().parse().unwrap()),
                    _ => panic!("unknown event"),
                }
            };
            (time, event)
        })
        .sorted_unstable_by_key(|(time, _)| *time)
        .scan(None, |guard_id, (time, event)| {
            if let Event::BeginShift(new_guard_id) = event {
                *guard_id = Some(new_guard_id);
            }
            Some((
                usize::try_from(time.rem_euclid(60)).unwrap(),
                guard_id.expect("guard began shift"),
                event,
            ))
        })
        .chunk_by(|(_, guard_id, event)| {
            let is_sleep_cycle = event == &Event::WakeUp || event == &Event::FallAsleep;
            (is_sleep_cycle, *guard_id)
        })
        .into_iter()
        .filter_map(|((is_sleep_cycle, guard_id), data)| {
            is_sleep_cycle.then(|| {
                let sleep_ranges = data
                    .map(|(minute, ..)| minute)
                    .tuples()
                    .map(|(fall_asleep_minute, wake_up_minute)| fall_asleep_minute..wake_up_minute);
                (guard_id, sleep_ranges)
            })
        })
        .into_grouping_map_by(|(guard_id, _)| *guard_id)
        .fold([0; 60], |mut minute_counts, _, (_, sleep_ranges)| {
            for minute in sleep_ranges.flatten() {
                minute_counts[minute] += 1;
            }
            minute_counts
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
            let (max_minute, count) = minute_counts
                .into_iter()
                .enumerate()
                .max_by_key(|(_, count)| *count)
                .unwrap();
            (guard_id, max_minute, count)
        })
        .max_by_key(|(.., count)| *count)
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

    const INPUT: &str = include_str!("../test_data/day_04.txt");

    #[test_case(EXAMPLE => 240)]
    #[test_case(INPUT => 94040)]
    fn part_1(input: &str) -> usize {
        super::part_1(input)
    }

    #[test_case(EXAMPLE => 4455)]
    #[test_case(INPUT => 39940)]
    fn part_2(input: &str) -> usize {
        super::part_2(input)
    }
}
