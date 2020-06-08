pub mod automata;
pub mod horspool;
pub mod two_way_alg;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, BufRead};
use std::time::{Duration, Instant};

pub fn hello() {
    println!("Hello world");
}

pub struct TestResult {
    pub positions: Vec<usize>,
    pub pattern: String,
    pub average_duration: Duration
}

pub fn test_by_files<F>(strings_file: &str, find_in_file: &str, matcher: F, times: usize) -> Vec<TestResult>
where F: Fn(&str, &str) -> Vec<usize>
{
    let mut strings = File::open(strings_file).unwrap();
    let mut find_in = File::open(find_in_file).unwrap();
    let mut text = String::new();
    let mut patterns = String::new();
    find_in.read_to_string(&mut text);
    strings.read_to_string(&mut patterns);

    let mut result = Vec::new();

    for pattern in patterns.lines() {
        let mut durations = Vec::new();
        let mut positions = Vec::new();
        for _ in 0..times {
            let now = Instant::now();
            positions = matcher(pattern, &text);
            durations.push(now.elapsed());
        }
        let average_duration: Duration = durations.iter().sum::<Duration>() / durations.len() as u32;
        let pattern = String::from(pattern);
        result.push(TestResult {
            positions,
            pattern,
            average_duration
        });
    }
    result
}