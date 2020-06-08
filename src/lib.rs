mod automata;
mod horspool;
mod two_way_alg;
use std::collections::HashSet;

pub fn hello() {
    let matching_str = "123";
    let find_in = "333123";
    let index = two_way_alg::tw(matching_str, find_in).unwrap();
    println!("{}", index);
}

