use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day7;
mod day9;

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
    let lines: Vec<String> = read_lines("data/day9.txt").into_iter().map(|x| x.unwrap()).collect();

    day9::go(lines);
}
