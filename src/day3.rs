fn binchar2int(c: &char) -> i32 {
    match c {
        '0' => 0,
        '1' => 1,
        _ => -1,
    }
}

pub fn go3(numbers: Vec<String>) {
    const BITS: usize = 12;

    let mut arr = vec![vec![0; numbers.len()]; BITS];
    for j in 0..BITS {
        for i in 0..numbers.len() {
            let digit = numbers[i].chars().nth(j).expect("Expected more digits");
            arr[j][i] = binchar2int(&digit);
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for j in 0..BITS {
        let sum = arr[j].iter().sum::<i32>();
        let half_num = (arr[j].len() / 2).try_into().unwrap();
        if sum > half_num {
            gamma += 1 << (BITS - j - 1);
        }
        else {
            epsilon += 1 << (BITS - j - 1);
        }
    }
    let power = gamma * epsilon;
    println!("Power: {}", power);
}
