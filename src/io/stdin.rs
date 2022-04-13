use std::io;
use std::io::{BufRead, Read};

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

pub fn has_next_line() -> bool {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines().map(|l| l.unwrap());
    match lines_iter.next() {
        None => {false}
        Some(_) => {true}
    }
}

// pub fn is_empty() -> bool {
//
// }

struct CachedData {
    data: String,
    index: usize
}

#[cfg(test)]
mod tests {
    use crate::io::stdin;

    #[test]
    fn read_line() {
        let text = stdin::read_line();
        println!("{}", text);
    }
}