use std::ops::Index;

pub fn horspool(needle: &str, haystack: &str) -> Option<usize> {
    let mut t = [0; 256];
    preprocess(needle, &mut t);
    let mut skip = 0;

    let mut chars_iter = haystack.chars();
    while let Some(_) = chars_iter.next() {
        if skip + needle.len() > haystack.len() {
            return None;
        }
        if &haystack[skip..skip + needle.len()] == needle {
            return Some(skip);
        }
        let mut iter_copy = chars_iter.clone();
        let char = iter_copy.nth(needle.len() - 2);
        if let Some(c) = char {
            let to_skip = t[c as u8 as usize] as usize;
            skip += to_skip;
            let skipped = Some(c);
            if to_skip > 1 {
                let skipped = chars_iter.nth(to_skip - 2);
            }
            // println!("{} {:?} {:?} {}", c, skipped, to_skip, skip);
        }
    }

    /*
    while haystack.len() - skip >= needle.len() {
        if &haystack[skip..skip + needle.len()] == needle {
            return Some(skip);
        }
        let char = haystack.chars().nth(skip + needle.len() - 1);
        if let Some(c) = char {
            skip += t[c as u8 as usize] as usize;
        }
    }
    */

    None
}

fn preprocess<'a>(pattern: &str, arr: &mut [u8]) {
    for (_, el) in arr.iter_mut().enumerate() {
        *el = pattern.len() as u8;
    }
    for (i, el) in pattern.chars().take(pattern.len() - 1).enumerate() {
        arr[el as usize] = (pattern.len() - i - 1) as u8;
    }
}

pub fn horspool_indexes(needle: &str, haystack: &str) -> Vec<usize> {
    let mut current_index = 0;
    let mut result = Vec::new();
    while current_index + needle.len() < haystack.len() {
        if let Some(index) = horspool(needle, &haystack[current_index..]) {
            result.push(current_index + index);
            current_index += index + needle.len();
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests_horspool {
    use crate::horspool::{horspool, horspool_indexes};

    #[test]
    fn test_casting() {
        let i = '0';
        let b = i as usize;
        assert_eq!(b, 48);
    }

    #[test]
    fn test_horspool_1() {
        let x = "123";
        let y = "12123";
        let b = horspool(x, y);
        assert_eq!(b, Some(2));
    }

    #[test]
    fn test_horspool_2() {
        let matching_str = "123";
        let find_in = "52230123";
        let index = horspool(matching_str, find_in).unwrap();
        assert_eq!(index, 5);
    }

    #[test]
    fn test_horspool_indexes_1() {
        let matching_str = "123";
        let find_in = "51230123012372317123";
        let indexes = horspool_indexes(matching_str, find_in);
        assert_eq!(indexes, vec![1, 5, 9, 17]);
    }

    #[test]
    fn test_horspool_indexes() {
        let matching_str = "123";
        let find_in = "12312312123";
        let indexes = horspool_indexes(matching_str, find_in);
        assert_eq!(indexes, vec![0, 3, 8]);
    }
}