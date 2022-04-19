/*
Tests the io parts of the library.
 */
use algs4_rust::context::convex_hull::{graham_scan, Point};
use algs4_rust::io;

fn main() {
    //    Use of any of the method to test the stdin functionality.
    //     cargo run < {file.txt}
    let n = read_int();
    let mut points: Vec<Point> = vec![];
    for _ in 0..n {
        let x = read_int();
        let y = read_int();
        let point = Point::new(x as f64, y as f64);
        points.push(point);
    }
    let graham_scan = graham_scan(&mut points);
    println!("{:?}", graham_scan);
}

fn read_line() -> String {
    io::stdin::read_line()
}

fn read_all_lines() -> Vec<String> {
    io::stdin::read_all_lines()
}

fn has_next_line() -> bool {
    io::stdin::has_next_line()
}

fn is_empty() -> bool {
    io::stdin::is_empty()
}

fn read_all_strings() -> Vec<String> {
    io::stdin::read_all_strings()
}

fn read_all() -> String {
    io::stdin::read_all()
}

fn read_all_ints() -> Vec<i32> {
    io::stdin::read_all_ints()
}

fn read_all_longs() -> Vec<i64> {
    io::stdin::read_all_longs()
}

fn read_all_doubles() -> Vec<f64> {
    io::stdin::read_all_doubles()
}

fn read_string() -> String {
    io::stdin::read_string()
}

fn read_boolean() -> bool {
    io::stdin::read_boolean()
}

fn read_int() -> i32 {
    io::stdin::read_int()
}

fn read_double() -> f64 {
    io::stdin::read_double()
}

fn read_byte() -> Vec<u8> {
    io::stdin::read_byte()
}
