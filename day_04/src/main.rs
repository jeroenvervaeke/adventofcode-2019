fn main() {
    let range = MIN..MAX;

    let count = range
        .filter_map(|x| if is_valid(x) { Some(x) } else { None })
        .count();

    println!("Number of combinations: {}", count);
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
}
