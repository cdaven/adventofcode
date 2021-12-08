use std::collections::HashSet;

pub fn go7a(numbers_str: &str) {
    let mut numbers: Vec<i32> = numbers_str
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    numbers.sort();
    let median = numbers[numbers.len() / 2];
    println!("Median {}", median);

    let fuel_values: i32 = numbers
        .iter()
        .map(|&n| (n - median).abs())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("Fuel sum {}", fuel_values);
}

fn fuel(distance: i32) -> i32 {
    if distance == 0 {
        0
    } else {
        (1..(distance + 1))
            .into_iter()
            .reduce(|acc, n| acc + n)
            .unwrap()
    }
}

fn calc_total_fuel(numbers: &Vec<i32>, target: i32) -> i32 {
    numbers
        .iter()
        .map(|n| fuel((n - target).abs()))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn find_minimum_fuel(numbers: &Vec<i32>) -> i32 {
    let average = (numbers.iter().sum::<i32>() as f32 / numbers.len() as f32).round();
    let min = (average * 0.8) as i32;
    let max = (average * 1.2) as i32;
    (min..max)
        .map(|n| {
            let total_cost = calc_total_fuel(numbers, n);
            total_cost
        })
        .min()
        .unwrap()
}

pub fn go7b(numbers_str: &str) {
    let numbers: Vec<i32> = numbers_str
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let fuel_sum = find_minimum_fuel(&numbers);
    println!("Minimum fuel sum {}", fuel_sum);
}
