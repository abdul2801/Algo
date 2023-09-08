pub fn is_valid_partition(square: u32) -> bool {
    let square_str = square.to_string();
    let mut sum = 0;
    let mut num_str = String::new();

    for ch in square_str.chars() {
        num_str.push(ch);
        let num = num_str.parse::<u32>().unwrap();
        sum += num;

        if sum > square {
            return false;
        } else if sum == square {
            return true;
        }
    }

    false
}

pub(crate) fn punishment_number(n: u32) -> u32 {
    let mut punishment = 0;

    for i in 1..=n {
        let square = i * i;
        if is_valid_partition(square) {
            punishment += square;
        }
    }

    punishment
}


