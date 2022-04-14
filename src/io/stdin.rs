use std::cell::RefCell;
use std::io;
use std::io::BufRead;

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

pub fn has_next_line() -> bool {
    let stdin = io::stdin();
    let mut lines_iter = stdin.lock().lines().map(|l| l.unwrap());
    match lines_iter.next() {
        None => { false }
        Some(_) => { true }
    }
}

fn initialize_buffer() -> InputBuffer {
    InputBuffer {
        lines: read_all_lines(),
        position: 0,
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

struct InputBuffer {
    lines: Vec<String>,
    position: usize,
}

#[cfg(test)]
mod tests {
    use crate::io::stdin;

    #[test]
    fn is_empty() {
    }
}