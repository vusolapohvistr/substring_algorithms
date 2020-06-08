use substring_algorithms::{test_by_files, hello, TestResult};
use substring_algorithms::automata::*;
use substring_algorithms::two_way_alg::*;
use substring_algorithms::horspool::*;

fn main() {
    /*
    let matching_str = "abc";
    let find_in = "wbbczabc";
    let index = horspool(matching_str, find_in);
    assert_eq!(index, Some(5));
    */

    let result = complex_test("test1-targets.txt", "test1.txt");
    println!("{}", result);
}

fn complex_test(targets_file: &str, find_in_file: &str) -> String {
    let mut result = String::new();
    let automata_results = test_by_files(targets_file, find_in_file, &automata_indexes, 10);
    automata_results.iter().for_each(|test_result| {
        result += format_result(test_result).as_str();
    });
    let horspool_results = test_by_files(targets_file, find_in_file, &horspool_indexes, 10);
    horspool_results.iter().for_each(|test_result| {
        result += format_result(test_result).as_str();
    });
    result
}

fn format_result(result: &TestResult) -> String {
    format!("<<{}>> {:?} \n {:?}s \n",
            result.pattern, result.positions, result.average_duration.as_secs_f64())
}