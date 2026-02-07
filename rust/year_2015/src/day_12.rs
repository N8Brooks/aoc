use Value::*;
use hashbrown::HashMap;
use itertools::Itertools as _;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Value<'a> {
    Arr(Vec<Value<'a>>),
    Obj(HashMap<&'a str, Value<'a>>),
    Num(i32),
    Str(&'a str),
}

pub fn part_1(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).unwrap();
    let mut stack = vec![&json];
    let mut sum = 0;
    while let Some(value) = stack.pop() {
        match value {
            Arr(arr) => stack.extend(arr),
            Obj(obj) => stack.extend(obj.values()),
            Num(n) => sum += n,
            Str(_) => {}
        }
    }
    sum
}

pub fn part_2(input: &str) -> i32 {
    let json: Value = serde_json::from_str(input).unwrap();
    let mut stack = vec![&json];
    let mut sum = 0;
    while let Some(value) = stack.pop() {
        match value {
            Arr(arr) => stack.extend(arr),
            Obj(obj) if !obj.values().contains(&Str("red")) => stack.extend(obj.values()),
            Num(n) => sum += n,
            _ => {}
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    const INPUT: &str = include_str!("../test_data/day_12.txt");

    #[test_case(r#"[1,2,3]"# => 6)]
    #[test_case(r#"{"a":2,"b":4}"# => 6)]
    #[test_case(r#"[[[3]]]"# => 3)]
    #[test_case(r#"{"a":{"b":4},"c":-1}"# => 3)]
    #[test_case(r#"{"a":[-1,1]}"# => 0; "example 5")]
    #[test_case(r#"[-1,{"a":1}]"# => 0; "example 6")]
    #[test_case(r#"[]"# => 0; "example 7")]
    #[test_case(r#"{}"# => 0; "example 8")]
    #[test_case(INPUT => 191164)]
    fn part_1(input: &str) -> i32 {
        super::part_1(input)
    }

    #[test_case(r#"[1,2,3]"# => 6)]
    #[test_case(r#"[1,{"c":"red","b":2},3]"# => 4)]
    #[test_case(r#"{"d":"red","e":[1,2,3,4],"f":5}"# => 0)]
    #[test_case(r#"{"a":"red","b":2,"c":3}"# => 0)]
    #[test_case(INPUT => 87842)]
    fn part_2(input: &str) -> i32 {
        super::part_2(input)
    }
}
