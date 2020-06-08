use std::cmp::max;

fn max_suf(x: &str, p: &mut usize) -> isize {
    let mut ms: isize = -1;
    let m = x.len();

    let mut j = 0;
    let mut k = 1;
    *p = 1;
    while j + k < m {
        let a = x.chars().skip(j + k).take(1).last().unwrap() as u8;
        let b = x.chars().skip((ms + k as isize)  as usize).take(1).last().unwrap() as u8;

        if a < b {
            j += k;
            k = 1;
            *p = (j as isize - ms) as usize;
        } else {
            if a == b {
                if k != *p {
                    k += 1;
                } else {
                    j += *p;
                    k = 1;
                }
            } else {
                ms = j as isize;
                j = (ms as usize) + 1;
                k = 1;
                *p = 1;
            }
        }
    }
    ms
}

fn max_suf_tilde(x: &str, p: &mut usize) -> isize {
    let mut ms: isize = -1;
    let m = x.len();

    let mut j = 0;
    let mut k = 1;
    *p = 1;

    while j + k < m {
        let a = x.chars().skip(j + k).take(1).last().unwrap() as u8;
        let b = x.chars().skip((ms + k as isize)  as usize).take(1).last().unwrap() as u8;

        if a > b {
            j += k;
            k = 1;
            *p = (j as isize - ms) as usize;
        } else {
            if a == b {
                if k != *p {
                    k += 1;
                } else {
                    j += *p;
                    k = 1;
                }
            } else {
                ms = j as isize;
                j = ms as usize + 1;
                k = 1;
                *p = 1;
            }
        }
    }
    ms
}

pub fn tw(x: &str, y: &str) -> Option<usize> {
    let m = x.len();
    let n = y.len();

    let mut p = 1;
    let mut q = 1;
    let mut ell;
    let mut per;
    let mut memory: isize;

    let mut i = max_suf(x, &mut p);
    let mut j = max_suf_tilde(x, &mut q);

    if i > j {
        ell = i;
        per = p;
    } else {
        ell = j;
        per = q;
    }

    // Searching
    if &x[..ell as usize + 1] == &x[per..per + ell as usize + 1] {
        j = 0;
        memory = -1;
        while j <= n as isize - m as isize {
            i = max(ell, memory) + 1;
            while i < m as isize &&
                x.chars().skip(i as usize).take(1).last().unwrap()
                    == y.chars().skip((i + j) as usize).take(1).last().unwrap() {
                i += 1;
            }
            if i >= m as isize{
                i = ell;
                while i > memory &&
                    x.chars().skip(i as usize).take(1).last().unwrap()
                        == y.chars().skip((i + j) as usize).take(1).last().unwrap() {
                    i -= 1;
                }
                if i < memory {
                    return Some(j as usize);
                }
                j += per as isize;
                memory = m as isize - per as isize - 1;
            } else {
                j += i - ell;
                memory = -1;
            }
        }
    } else {
        per = max(ell as usize + 1, (m as isize - ell - 1) as usize) + 1;
        j = 0;
        while j <= n as isize - m as isize {
            i = ell + 1;
            while i < m as isize &&
                x.chars().skip(i as usize).take(1).last().unwrap()
                    == y.chars().skip((i + j) as usize).take(1).last().unwrap() {
                i += 1;
            }
            if i >= m as isize {
                i = ell;
                while i >= 0 &&
                    x.chars().skip(i as usize).take(1).last().unwrap()
                        == y.chars().skip((i + j) as usize).take(1).last().unwrap() {
                    i -= 1;
                }
                if i < 0 {
                    return Some(j as usize);
                }
                j += per as isize;
            } else {
                j += (i - ell);
            }
        }
    }
    None
}

pub fn tw_indexes(needle: &str, haystack: &str) -> Vec<usize> {
    let mut current_index = 0;
    let mut result = Vec::new();
    while current_index + needle.len() < haystack.len() {
        if let Some(index) = tw(needle, &haystack[current_index..]) {
            result.push(current_index + index);
            current_index += index + needle.len();
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests_tw {
    use super::*;

    #[test]
    fn test_tw_1() {
        let matching_str = "123";
        let find_in = "333123";
        let b = tw(matching_str, find_in);
        assert_eq!(b, Some(3));
    }

    #[test]
    fn test_tw_2() {
        let matching_str = "123";
        let find_in = "52230123012372317123";
        let index = tw(matching_str, find_in).unwrap();
        assert_eq!(index, 5);
    }

    #[test]
    fn test_tw_indexes_1() {
        let matching_str = "123";
        let find_in = "51230123012372317123";
        let indexes = tw_indexes(matching_str, find_in);
        assert_eq!(indexes, vec![1, 5, 9, 17]);
    }

    #[test]
    fn test_tw_indexes() {
        let matching_str = "123";
        let find_in = "12312312123";
        let indexes = tw_indexes(matching_str, find_in);
        assert_eq!(indexes, vec![0, 3, 8]);
    }
}