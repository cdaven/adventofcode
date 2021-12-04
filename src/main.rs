use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day1;
mod day2;
mod day3;

pub use day1::*;
pub use day2::*;
pub use day3::*;

/// Read file into lines.
/// Copied from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn main() {
    let lines: Vec<String> = read_lines("data/day3.txt").into_iter().map(|x| x.unwrap()).collect();

    go3b(lines);
}
