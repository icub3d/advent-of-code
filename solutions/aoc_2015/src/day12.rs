const INPUT: &'static str = include_str!("inputs/day12.txt");

pub fn p1(input: &Value) -> i64 {
    match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(p1).sum(),
        Value::Object(o) => o.values().map(p1).sum(),
        _ => 0,
    }
}

pub fn p2(input: &Value) -> i64 {
    match input {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(p2).sum(),
        Value::Object(o) => {
            // If we have a "red" value, return 0 instead.
            if o.values()
                .any(|v| matches!(v, Value::String(s) if s == "red"))
            {
                0
            } else {
                o.values().map(p2).sum()
            }
        }
        _ => 0,
    }
}

use std::time::Instant;

use serde_json::Value;

pub fn solve() -> anyhow::Result<()> {
    let input: Value = serde_json::from_str(INPUT)?;
    let now = Instant::now();
    println!("p1: {} ({:?})", p1(&input), now.elapsed());
    let now = Instant::now();
    println!("p2: {} ({:?})", p2(&input), now.elapsed());
    Ok(())
}
