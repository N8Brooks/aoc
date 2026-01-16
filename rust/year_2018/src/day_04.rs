use std::collections::HashMap;

use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let (id, counts) = parse_input(input)
        .into_iter()
        .max_by_key(|(_, counts)| counts.iter().sum::<usize>())
        .unwrap();
    let (minute, _) = counts
        .into_iter()
        .enumerate()
        .max_by_key(|(_, count)| *count)
        .unwrap();
    id * minute
}

pub fn part_2(input: &str) -> usize {
    let (id, minute, _) = parse_input(input)
        .into_iter()
        .map(|(id, counts)| {
            let (minute, count) = counts
                .into_iter()
                .enumerate()
                .max_by_key(|(_, count)| *count)
                .unwrap();
            (id, minute, count)
        })
        .max_by_key(|(.., count)| *count)
        .unwrap();
    id * minute
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

use Event::*;

fn parse_input(input: &str) -> HashMap<usize, [usize; 60]> {
    input
        .lines()
        .map(|line| {
            let (timestamp, event) = (&line[..18], &line[19..]);
            let timestamp = timestamp.strip_circumfix('[', ']').unwrap();
            let event = if event == "falls asleep" {
                FallAsleep
            } else if event == "wakes up" {
                WakeUp
            } else if let Some(id) = event.strip_circumfix("Guard #", " begins shift") {
                BeginShift(id.parse().unwrap())
            } else {
                panic!("unknown event: {}", event);
            };
            (timestamp, event)
        })
        .sorted_unstable_by(|(a, _), (b, _)| a.cmp(b))
        .scan(None, |id, (timestamp, event)| {
            if let BeginShift(id2) = event {
                *id = Some(id2);
            }
            Some((id.expect("guard began shift"), timestamp, event))
        })
        .chunk_by(|(id, ..)| *id)
        .into_iter()
        .into_grouping_map_by(|(id, _)| *id)
        .fold([0; 60], |counts, _, (_, data)| {
            data.filter(|(.., event)| !matches!(event, BeginShift(_)))
                .map(|(_, timestamp, event)| {
                    let (_, minute) = timestamp.rsplit_once(':').unwrap();
                    let minute: usize = minute.parse().unwrap();
                    (minute, event)
                })
                .tuples()
                .flat_map(|((sleep, event_1), (wake, event_2))| {
                    assert_eq!(event_1, FallAsleep);
                    assert_eq!(event_2, WakeUp);
                    sleep..wake
                })
                .fold(counts, |mut counts, minute| {
                    counts[minute] += 1;
                    counts
                })
        })
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
