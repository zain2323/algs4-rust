/*
Tests the io parts of the library.
 */
use std::io::stdin;

pub mod io;

fn main() {
//    Use of any of the method to test the stdin functionality.
//     cargo run < {file.txt}
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

fn read_all_strings() {
    let all_strings = io::stdin::read_all_strings();
    println!("{:?}", all_strings);
}

fn read_all() {
    let all_strings = io::stdin::read_all();
    println!("{}", all_strings);
}

fn read_all_ints() {
    let all_ints = io::stdin::read_all_ints();
    println!("{:?}", all_ints);
}

fn read_all_longs() {
    let all_longs = io::stdin::read_all_longs();
    println!("{:?}", all_longs);
}

fn read_all_doubles() {
    let all_doubles = io::stdin::read_all_doubles();
    println!("{:?}", all_doubles);
}

