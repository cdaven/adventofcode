use std::clone::Clone;

fn binchar2int(c: &char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        _ => -1,
    }
}

// const BITS: usize = 5; // For sample data
const BITS: usize = 12; // For real data

fn numbers_to_binary_array(numbers: &Vec<String>) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![0; numbers.len()]; BITS];
    for j in 0..BITS {
        for i in 0..numbers.len() {
            let digit = numbers[i].chars().nth(j).expect("Expected more digits");
            arr[j][i] = binchar2int(&digit);
        }
    }
    arr
}

pub fn go3a(numbers: Vec<String>) {
    let arr = numbers_to_binary_array(&numbers);
    let mut gamma = 0;
    let mut epsilon = 0;
    for j in 0..BITS {
        let sum = arr[j].iter().sum::<i32>();
        let half_num = (arr[j].len() / 2).try_into().unwrap();
        if sum > half_num {
            gamma += 1 << (BITS - j - 1);
        } else {
            epsilon += 1 << (BITS - j - 1);
        }
    }
    let power = gamma * epsilon;
    println!("Power: {}", power);
}

/** Get elements from array as indexed by vector */
fn indices<T: Clone>(arr: &Vec<T>, ix: &Vec<usize>) -> Vec<T> {
    let mut a = Vec::new();
    for i in ix {
        a.push(arr.as_slice()[*i].clone());
    }
    a
}

fn get_most_common(arr: &Vec<i32>) -> i32 {
    let ones: Vec<&i32> = arr.iter().filter(|&i| *i == 1).collect();
    let zeroes: Vec<&i32> = arr.iter().filter(|&i| *i == 0).collect();
    if ones.len() == 0 {
        0
    } else if zeroes.len() == 0 {
        1
    } else if ones.len() >= zeroes.len() {
        1
    } else {
        0
    }
}

fn get_least_common(arr: &Vec<i32>) -> i32 {
    let ones: Vec<&i32> = arr.iter().filter(|&i| *i == 1).collect();
    let zeroes: Vec<&i32> = arr.iter().filter(|&i| *i == 0).collect();
    if ones.len() == 0 {
        0
    } else if zeroes.len() == 0 {
        1
    } else if ones.len() < zeroes.len() {
        1
    } else {
        0
    }
}

pub fn go3b(numbers: Vec<String>) {
    let arr = numbers_to_binary_array(&numbers);
    let mut oxygen_indices: Vec<usize> = (0..numbers.len()).collect();
    let mut co2_indices: Vec<usize> = (0..numbers.len()).collect();

    while oxygen_indices.len() > 1 || co2_indices.len() > 1 {
        for j in 0..BITS {
            if oxygen_indices.len() > 1 {
                let oxygen_01 = get_most_common(&indices::<i32>(&arr[j], &oxygen_indices));
                oxygen_indices.retain(|&i| arr[j][i] == oxygen_01);
            }
            if co2_indices.len() > 1 {
                let co2_01 = get_least_common(&indices::<i32>(&arr[j], &co2_indices));
                co2_indices.retain(|&i| arr[j][i] == co2_01);
            }
        }
    }

    println!(
        "Oxygen: {}",
        isize::from_str_radix(&numbers[oxygen_indices[0]], 2).unwrap()
    );
    println!(
        "CO2: {}",
        isize::from_str_radix(&numbers[co2_indices[0]], 2).unwrap()
    );
}
