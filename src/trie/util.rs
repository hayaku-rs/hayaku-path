use super::reg::Regex;
use regex;

pub fn end_of_param(path: &str) -> String {
    match path.find('/') {
        Some(i) => path[i..].to_string(),
        None => String::new(),
    }
}

pub fn parse_param(param: &str) -> (String, Regex) {
    let mut param_name = String::new();
    let mut regex = String::new();
    let mut in_param = true;
    for c in param.chars() {
        match c {
            ':' => {
                if in_param {
                    in_param = false;
                } else {
                    regex.push(c);
                }
            }
            c => {
                if in_param {
                    param_name.push(c);
                } else {
                    regex.push(c);
                }
            }
        }
    }
    if regex.is_empty() {
        regex = ".+".to_string();
    }
    let regex = regex::Regex::new(&regex).unwrap();
    let regex = Regex(regex);
    (param_name, regex)
}

/// Determines the length of the shared prefix of two strings.
/// E.g. `get_match_len("apple", "ape") => 2`.
pub fn get_match_len(a: &str, b: &str) -> usize {
    let mut match_len = 0;
    for (ac, bc) in a.chars().zip(b.chars()) {
        if ac == bc && bc != '{' {
            match_len += 1;
        } else {
            break;
        }
    }
    match_len
}

pub fn get_params_indices(path: &str) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();
    let mut start = 0;
    let mut braces = 0usize;
    for (i, c) in path.char_indices() {
        match c {
            '{' => {
                if braces == 0 {
                    start = i;
                }
                braces += 1;
            }
            '}' => {
                braces -= 1;
                if braces == 0 {
                    indices.push((start, i + 1));
                }
            }
            _ => {}
        }
    }
    // If `in_param`, there is a missing `}`.
    assert_eq!(braces, 0);
    indices
}

pub fn splitn(s: &str, n: usize, pat: char) -> Vec<String> {
    let mut n = n;
    let mut strings = Vec::new();
    let mut buf = String::new();

    for ch in s.chars() {
        if n <= 1 {
            buf.push(ch);
            continue;
        }
        if ch == pat {
            n -= 1;
            strings.push(buf);
            buf = String::new();
            buf.push(ch);
        } else {
            buf.push(ch);
        }
    }
    strings.push(buf);
    strings
}
