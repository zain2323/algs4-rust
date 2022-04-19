use algs4_rust::fundamentals::indexed_pq;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut args = std::env::args();
    let mut stream: Vec<File> = vec![];
    for arg in args {
        stream.push(File::open(arg).unwrap());
    }

    merge(stream);
}

fn merge(mut streams: Vec<File>) {
    let n = streams.len();
    let mut heap = indexed_pq::MinHeap::<String>::new();
    let mut contents = String::new();
    for i in 0..n {
        streams[i].read_to_string(&mut contents);
    }
    let mut chars = contents.chars().filter(|c| *c != ' ').into_iter();

    for i in 0..n {
        let content = chars.next();
        if !content.is_none() {
            heap.insert(i, content.unwrap().to_string());
        }
    }

    while !heap.is_empty() {
        println!("{}", heap.min());
        let i = heap.del_min();
        let content = chars.next();
        if !content.is_none() {
            heap.insert(i, content.unwrap().to_string());
        }
    }
}
