use std::collections::HashMap;
use chrono::{DateTime, Datelike};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let iso_week = date.iso_week();
                let week_str = format!("{}-W{}", iso_week.year(), iso_week.week());
                *map.entry(week_str).or_insert(0) += 1;
            }
        }
    }

    map
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *map.entry(login.to_string()).or_insert(0) += 1;
        }
    }

    map
}
