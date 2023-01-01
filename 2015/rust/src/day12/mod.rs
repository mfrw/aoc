use crate::utils;
use serde_json::Value;

pub struct Day;

impl utils::Solver<12> for Day {
    type Part1 = i64;

    type Part2 = i64;

    fn part1(&self, i: &str) -> Result<Self::Part1, Box<dyn std::error::Error>> {
        let v = Json {
            v: serde_json::from_str(i)?,
        };

        Ok(v.eval())
    }

    fn part2(&self, i: &str) -> Result<Self::Part2, Box<dyn std::error::Error>> {
        let v = Json {
            v: serde_json::from_str(i)?,
        };
        Ok(v.eval2())
    }
}

struct Json {
    v: Value,
}

impl Json {
    fn eval(&self) -> i64 {
        eval_val(&self.v)
    }
    fn eval2(&self) -> i64 {
        eval_val2(&self.v)
    }
}

fn eval_val2(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(v) => v.into_iter().map(|v| eval_val2(v)).sum(),
        Value::Object(o) => {
            if o.values()
                .any(|v| v.is_string() && v.as_str() == Some("red"))
            {
                0
            } else {
                o.into_iter().map(|(_, v)| eval_val2(v)).sum()
            }
        }
    }
}

fn eval_val(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(v) => v.into_iter().map(|v| eval_val(v)).sum(),
        Value::Object(o) => o.into_iter().map(|(_, v)| eval_val(v)).sum(),
    }
}

#[test]
fn test_eval2_array() {
    let s = "[1,2,3]";
    let v = serde_json::from_str(s).unwrap();
    assert_eq!(6, eval_val2(&v));
}

#[test]

fn test_eval2_array_with_str() {
    let s = r#"[1,"red",5]"#;
    let v = serde_json::from_str(s).unwrap();
    assert_eq!(6, eval_val2(&v));
}

#[test]
fn test_eval2_array_with_obj() {
    let s = r#"[1,{"c":"red","b":2},3]"#;
    let v = serde_json::from_str(s).unwrap();
    assert_eq!(4, eval_val2(&v));
}

#[test]
fn test_eval2_obj() {
    let s = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
    let v = serde_json::from_str(s).unwrap();
    assert_eq!(0, eval_val2(&v));
}
