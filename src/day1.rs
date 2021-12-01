use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

/// Read file into lines.
/// Copied from https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Count number of elements in vector whose value is larger than the previous
fn count_increases(v: Vec<i32>) -> i32 {
    v.iter()
        // Fold with a tuple as initial value
        .fold((0, i32::MAX), |acc, &x| (acc.0 + ((x > acc.1) as i32), x))
        .0
}

pub fn day1a() {
    let lines = read_lines("data/day1.txt").expect("Some error reading file");
    let v: Vec<i32> = lines
        .into_iter()
        .map(|l| l.expect("Some error on some line").parse::<i32>().unwrap())
        .collect();

    let count = count_increases(v);
    println!("{}", count);
}

pub fn day1b() {
    let lines = read_lines("data/day1.txt").expect("Some error reading file");
    let v: Vec<i32> = lines
        .into_iter()
        .map(|l| l.expect("Some error on some line").parse::<i32>().unwrap())
        .collect();

    let mut measurements = v.into_iter();
    let mut sums: Vec<i32> = Vec::new();

    while measurements.len() >= 3 {
        let next3 = &measurements.as_slice()[0..3];
        //println!("\r\nNext 3");
        let mut sum = 0;
        for i in next3 {
            sum += i;
            //println!("{}", i);
        }

        //println!("Sum: {}", sum);
        sums.push(sum);
        // Consume first element in iterator
        measurements.next();
    }

    let count = count_increases(sums);
    println!("{}", count);
}
