use anchor_lang::prelude::*;

pub fn calculate_sqrt(value: u64) -> Result<u64> {
    let sqrt = integer_sqrt(value);
    Ok(sqrt)
}

fn integer_sqrt(value: u64) -> u64 {
    if value == 0 || value == 1 {
        return value;
    }

    let mut low = 0;
    let mut high = value;
    let mut result = 0;

    while low <= high {
        let mid = (low + high) / 2;
        if mid * mid == value {
            return mid;
        } else if mid * mid < value {
            low = mid + 1;
            result = mid;
        } else {
            high = mid - 1;
        }
    }

    result
}
