use std::collections::HashMap;

fn main() {
    let count = (MIN..MAX)
        .filter_map(|x| if is_valid(x) { Some(x) } else { None })
        .count();

    println!("Number of combinations: {}", count);

    let count2 = (MIN..MAX)
        .filter_map(|x| if is_valid_2(x) { Some(x) } else { None })
        .count();

    println!("[Part 2] Number of combinations: {}", count2);
}

const MIN: u32 = 124_075;
const MAX: u32 = 580_769;

fn is_valid(value: u32) -> bool {
    let chars: Vec<char> = value.to_string().chars().collect();

    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    chars
        .windows(2)
        .any(|cs| cs[0].to_digit(10) == cs[1].to_digit(10))
        && chars
            .windows(2)
            .all(|cs| cs[0].to_digit(10) <= cs[1].to_digit(10))
}

fn is_valid_2(value: u32) -> bool {
    if is_valid(value) {
        let mut frequency_map = HashMap::new();
        let chars: Vec<char> = value.to_string().chars().collect();

        for cs in chars.windows(2) {
            if cs[0] == cs[1] {
                frequency_map
                    .entry(cs[0])
                    .and_modify(|value| *value += 1)
                    .or_insert(2);
            }
        }

        frequency_map.values().any(|value| *value == 2)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_122345_valid() {
        assert_eq!(is_valid(122345), true)
    }

    #[test]
    fn example_111123_valid() {
        assert_eq!(is_valid(111123), true)
    }

    #[test]
    fn example_135679_invalid() {
        assert_eq!(is_valid(135679), false)
    }

    #[test]
    fn example_111111_valid() {
        assert_eq!(is_valid(111111), true)
    }

    #[test]
    fn example_223450_invalid() {
        assert_eq!(is_valid(223450), false)
    }

    #[test]
    fn example_123789_invalid() {
        assert_eq!(is_valid(123789), false)
    }

    #[test]
    fn example_part2_112233_valid() {
        assert_eq!(is_valid_2(112233), true)
    }

    #[test]
    fn example_part2_123444_invalid() {
        assert_eq!(is_valid_2(123444), false)
    }

    #[test]
    fn example_part2_111122_valid() {
        assert_eq!(is_valid_2(111122), true)
    }
}
