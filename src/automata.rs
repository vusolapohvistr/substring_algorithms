use std::collections::{HashMap, HashSet};
use std::ops::Index;
use crate::automata::State::{NextState, FinalState};

#[derive(Debug, Clone)]
enum State {
    NextState(usize),
    FinalState
}

#[derive(Debug, Clone)]
pub struct AutomatonTemplate {
    state_table: Vec<HashMap<char, State>>
}

impl AutomatonTemplate {
    pub fn for_string(arg: &str) -> AutomatonTemplate {
        let symbols_in_arg: HashSet<_> = arg.chars().collect();
        let mut state_table = Vec::with_capacity(arg.len());
        for (current_index, symbol) in arg.chars().enumerate() {
            let mut matcher = HashMap::new();
            for (i, current_char) in arg[0..=current_index].chars().enumerate() {
                let left_sub_word = &arg[0..=i];
                let right_sub_word = &arg[current_index - i..current_index];
                // println!("{} {} {} {}", left_sub_word, right_sub_word, i, current_index);
                for u_char in symbols_in_arg.iter() {
                    if &left_sub_word[0..i] == right_sub_word
                        && left_sub_word.chars().last().unwrap() == *u_char {
                        matcher.insert(current_char, NextState(i + 1));
                       // println!("{:?}", matcher);
                    }
                }
            }
            // println!("{:?}", matcher);
            if current_index == arg.len() - 1 {
                matcher.insert(symbol, FinalState);
            } else {
                matcher.insert(symbol, NextState(current_index + 1));
            }
            state_table.push(matcher);
        }
        AutomatonTemplate {
            state_table
        }
    }
    pub fn get_matcher(&self) -> Automaton {
        Automaton {
            automaton_template: self
        }
    }
}

#[derive(Debug, Clone)]
pub struct Automaton<'a> {
    automaton_template: &'a AutomatonTemplate
}

impl Automaton<'_> {
    pub fn test(&self, arg: &str) -> bool {
        if arg.len() < self.automaton_template.state_table.len() {
            return false
        }
        let mut state = 0;
        // println!("{:?}", self.automaton_template.state_table);
        for char in arg.chars() {
            state = match self.automaton_template.state_table[state].get(&char) {
                Some(val) => {
                    match val {
                        NextState(next_state) => *next_state,
                        FinalState => return true
                    }
                },
                None => 0
            };
        }
        false
    }

    pub fn indexes(&self, arg: &str) -> Vec<usize> {
        if arg.len() < self.automaton_template.state_table.len() {
            return vec![];
        }
        let mut result = Vec::new();
        let mut current_index = 0;
        let mut state = 0;
        for (index, char) in arg.chars().enumerate() {
            state = match self.automaton_template.state_table[state].get(&char) {
                Some(val) => {
                    match val {
                        NextState(next_state) => {
                            current_index = index + 1 - *next_state;
                            *next_state
                        },
                        FinalState => {
                            result.push(current_index);
                            current_index = index + 1;
                            0
                        }
                    }
                },
                None => {
                    current_index = index + 1;
                    0
                }
            };
        }
        result
    }
}

pub fn automata_indexes(needle: &str, haystack: &str) -> Vec<usize> {
    let matcher = AutomatonTemplate::for_string(needle);
    let automata = matcher.get_matcher();
    automata.indexes(haystack)
}

#[cfg(test)]
mod automata_tests {
    use super::*;

    #[test]
    fn test_matcher_1() {
        let matching_str = "12312";
        let find_in = "123124112415112312";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        assert!(automata.test(find_in));
    }

    #[test]
    fn test_matcher_2() {
        let matching_str = "dgidjnusso";
        let find_in = "rhrjrhjfrhfjrohhduhdgidjnussojsybusundvtdhnidiyudtuejidug";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        assert!(automata.test(find_in));
    }


    #[test]
    fn test_matcher_3() {
        let matching_str = "123";
        let find_in = "12123";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        assert!(automata.test(find_in));
    }

    #[test]
    fn test_matcher_fail_1() {
        let matching_str = "123";
        let find_in = "rhrjrhjfrhfjrohhduhdgidjnussojsybusundvtdhnidiyudtuejidug";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        assert!(!automata.test(find_in));
    }

    #[test]
    fn test_indexes_1() {
        let matching_str = "Allies";
        let find_in = "1Allies";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        let indexes = automata.indexes(find_in);
        assert_eq!(indexes, vec![1]);
    }

    #[test]
    fn test_indexes_2() {
        let matching_str = "123";
        let find_in = "51230123012372317123";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        let indexes = automata.indexes(find_in);
        assert_eq!(indexes, vec![1, 5, 9, 17]);
    }

    #[test]
    fn test_indexes_3() {
        let matching_str = "123";
        let find_in = "123123";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        let indexes = automata.indexes(find_in);
        assert_eq!(indexes, vec![0, 3]);
    }

    #[test]
    fn test_indexes_4() {
        let matching_str = "123";
        let find_in = "12312312123";
        let matcher = AutomatonTemplate::for_string(matching_str);
        let automata = matcher.get_matcher();
        let indexes = automata.indexes(find_in);
        assert_eq!(indexes, vec![0, 3, 8]);
    }
}