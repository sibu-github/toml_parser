use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, NaiveTime, Utc};

use crate::error::ParseError;
use crate::token::Token;

#[derive(Debug)]
pub enum Value {
    StringValue(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    DateTimeValue(DateTime<Utc>),
    TimeValue(NaiveTime),
    Array(Vec<Value>),
    InlineTable(HashMap<String, Value>),
    TableArray(Vec<Value>),
    Table(HashMap<String, Value>),
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.chars().map(Into::into).collect::<Vec<Token>>();
        Ok(Self::Boolean(false))
    }
}
