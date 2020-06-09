use substring_algorithms::{test_by_files, hello, TestResult};
use substring_algorithms::automata::*;
use substring_algorithms::two_way_alg::*;
use substring_algorithms::horspool::*;
use std::time::Duration;
use std::fs::File;
use rand::{thread_rng, Rng};
use std::io::Write;
use rand::distributions::{Standard, Alphanumeric};

fn main() -> std::io::Result<()> {
    /*
    let matching_str = "abc";
    let find_in = "wbbczabc";
    let index = horspool(matching_str, find_in);
    assert_eq!(index, Some(5));
    */
    let (test2_targets, test2) = generate_test_2(1000000);
    let mut test2_targets_file = File::create("test2-targets.txt")?;
    let mut test2_file = File::create("test2.txt")?;
    test2_targets_file.write(test2_targets.as_bytes());
    test2_file.write(test2.as_bytes());



    let result_test1 = complex_test("test1-targets.txt", "test1.txt");
    let mut result_test1_file = File::create("test1-results.txt")?;
    result_test1_file.write(result_test1.as_bytes());
    println!("{}", result_test1);

    let result_test2 = complex_test("test2-targets.txt", "test2.txt");
    let mut result_test2_file = File::create("test2-results.txt")?;
    result_test2_file.write(result_test1.as_bytes());
    println!("{}", result_test2);

    Ok(())
}

fn complex_test(targets_file: &str, find_in_file: &str) -> String {
    let mut result = String::from(format!("{}\n\n", find_in_file));

    result += "\nAutomata\n";
    let automata_results = test_by_files(targets_file, find_in_file, &automata_indexes, 10);
    automata_results.iter().for_each(|test_result| {
        result += format_result(test_result).as_str();
    });

    result += "\nhorspool\n";
    let horspool_results = test_by_files(targets_file, find_in_file, &horspool_indexes, 10);
    horspool_results.iter().for_each(|test_result| {
        result += format_result(test_result).as_str();
    });

    result += "\ntwo-way alg\n";
    let two_way_results = test_by_files(targets_file, find_in_file, &tw_indexes, 10);
    two_way_results.iter().for_each(|test_result| {
        result += format_result(test_result).as_str();
    });

    result += format!("Automata sum {}s\n",
                      (automata_results.iter()
                          .fold(
                              Duration::new(0, 0),
                              |mut sum, el| {
                                  sum += el.average_duration;
                                  sum
                              })).as_secs_f64()).as_str();

    result += format!("Horspool sum {}s\n",
                      (horspool_results.iter()
                          .fold(
                              Duration::new(0, 0),
                              |mut sum, el| {
                                  sum += el.average_duration;
                                  sum
                              })).as_secs_f64()).as_str();

    result += format!("Two way alg sum {}s\n",
                      (two_way_results.iter()
                          .fold(
                              Duration::new(0, 0),
                              |mut sum, el| {
                                  sum += el.average_duration;
                                  sum
                              })).as_secs_f64()).as_str();


    result
}

fn format_result(result: &TestResult) -> String {
    format!("<<{}>> {:?} \n {:?}s \n",
            result.pattern, result.positions, result.average_duration.as_secs_f64())
}

fn generate_test_2(size: usize) -> (String, String) {
    // text
    let mut rng = thread_rng();
    let mut find_in: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(|el| {
            if rng.gen_range(0, 30) == 20 {
                ' '
            } else {
                el
            }
        })
        .take(size)
        .collect();

    // matches
    let mut ended_line = false;
    let mut matching_strs: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(|el| {
            if !ended_line && rng.gen_range(0, 5) == 2 {
                ended_line = true;
                '\n'
            } else {
                ended_line = false;
                el
            }
        })
        .take(size / 1000)
        .collect::<String>().split('\n').collect();

    (matching_strs, find_in)
}