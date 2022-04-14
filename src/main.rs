/*
Tests the io parts of the library.
 */
pub mod io;

fn main() {
    is_empty();
    is_empty();
    is_empty();
    is_empty();
    is_empty();

}

fn read_line() {
    let lines = io::stdin::read_line();
    println!("{}", lines);
}

fn read_all_lines() {
    let lines = io::stdin::read_all_lines();
    println!("{:?}", lines);
}

fn has_next_line() {
    let boolean = io::stdin::has_next_line();
    println!("{}", boolean);
}

fn is_empty() {
    let boolean = io::stdin::is_empty();
    println!("{}", boolean);
}


