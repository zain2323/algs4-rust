use std::cell::RefCell;
use std::io;
use std::io::{BufRead};
use std::iter::Map;
use std::str::Split;

thread_local!(static CACHED_BUFFER: RefCell<InputBuffer> = RefCell::new(initialize_buffer()));

pub fn read_line() -> String {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(..) => buffer,
        Err(e) => panic!("{:?}", e)
    }
}

pub fn read_all_lines() -> Vec<String> {
    let mut all_lines: Vec<String> = vec![];
    for line_result in io::stdin().lock().lines() {
        match line_result {
            Ok(line) => all_lines.push(line),
            Err(e) => panic!("{:?}", e)
        }
    }
    all_lines
}

pub fn read_all_strings() -> Vec<String> {
    let lines = read_all_lines();
    let mut strings: Vec<String> = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        for string in split {
            strings.push(string.to_string());
        }
    }
    strings
}

pub fn read_all() -> String {
    let lines = read_all_lines();
    let mut all_strings = String::new();

    for line in lines {
        all_strings.push_str(&line);
        all_strings.push_str("\n");
    }
    all_strings.remove(all_strings.len()-1);
    all_strings
}

pub fn read_all_ints() -> Vec<i32> {
    let lines = read_all_lines();
    let mut all_ints: Vec<i32> = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        for int in split {
            if int.trim() != ""
            {
                let val = match int.trim().parse() {
                    Ok(v) => v,
                    Err(_) => panic!("Unable to parse the input ({}) to the 32bit integer", int.trim())
                };
                all_ints.push(val);
            }
        }
    }
    all_ints
}

pub fn read_all_longs() -> Vec<i64> {
    let lines = read_all_lines();
    let mut all_longs: Vec<i64> = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        for long in split {
            if long.trim() != ""
            {
                let val = match long.trim().parse() {
                    Ok(v) => v,
                    Err(_) => panic!("Unable to parse the input ({}) to the 64bit integer", long.trim())
                };
                all_longs.push(val);
            }
        }
    }
    all_longs
}

pub fn read_all_doubles() -> Vec<f64> {
    let lines = read_all_lines();
    let mut all_doubles: Vec<f64> = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        for double in split {
            if double.trim() != ""
            {
                let val = match double.trim().parse() {
                    Ok(v) => v,
                    Err(_) => panic!("Unable to parse the input ({}) to the 64bit floating point value", double.trim())
                };
                all_doubles.push(val);
            }
        }
    }
    all_doubles
}

pub fn has_next_line() -> bool {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines().map(|l| l.unwrap());
    match lines_iter.next() {
        None => { false }
        Some(_) => { true }
    }
}

pub fn is_empty() -> bool {
    let ans = CACHED_BUFFER.with(|buffer| {
        let mut borrowed_buffer = buffer.borrow_mut();
        if borrowed_buffer.position >= borrowed_buffer.lines.len() { true } else {
            println!("{:?}", borrowed_buffer.lines);
            let item = borrowed_buffer.lines.get(borrowed_buffer.position);
            match item {
                Some(_) => {
                    borrowed_buffer.position += 1;
                    false
                }
                None => true
            }
        }
    });
    ans
}

fn initialize_buffer() -> InputBuffer {
    InputBuffer {
        lines: read_all_lines(),
        position: 0,
    }
}


struct InputBuffer {
    lines: Vec<String>,
    position: usize,
}

#[cfg(test)]
mod tests {
    use crate::io::stdin;

    #[test]
    fn read_all_strings() {
        stdin::read_all_lines();
    }
}