use std::error::Error;
use std::time::Instant;

const BASE_PATTERN: [i16; 4] = [0, 1, 0, -1];

fn main() -> Result<(), Box<dyn Error>> {
    let input = to_i16_array("59791911701697178620772166487621926539855976237879300869872931303532122404711706813176657053802481833015214226705058704017099411284046473395211022546662450403964137283487707691563442026697656820695854453826690487611172860358286255850668069507687936410599520475680695180527327076479119764897119494161366645257480353063266653306023935874821274026377407051958316291995144593624792755553923648392169597897222058613725620920233283869036501950753970029182181770358827133737490530431859833065926816798051237510954742209939957376506364926219879150524606056996572743773912030397695613203835011524677640044237824961662635530619875905369208905866913334027160178")?;
    let now = Instant::now();
    let output: Vec<i16> = progress_n(input, 100).into_iter().take(8).collect();
    let diff = Instant::now() - now;

    println!("Output {:?}, {}ms", output, diff.as_millis());

    Ok(())
}

fn calculate_progress_matrix(len: usize) -> Vec<Vec<i16>> {
    let mut output = Vec::with_capacity(len);

    for idx in 0..len {
        output.push(BasePatternIterator::new(idx + 1).take(len).collect());
    }

    output
}

fn progress_with_matrix(input: &Vec<i16>, matrix: &Vec<Vec<i16>>) -> Vec<i16> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .zip(input.iter())
                .map(|(x, y)| x * y)
                .sum::<i16>()
                .abs()
                % 10
        })
        .collect()
}

fn progress_n(input: Vec<i16>, amount: u32) -> Vec<i16> {
    let matrix = calculate_progress_matrix(input.len());

    (0..amount).fold(input, |acc, _| {
        let next = progress_with_matrix(&acc, &matrix);
        next
    })
}

fn to_i16_array<T: ToString>(input: T) -> Result<Vec<i16>, Box<dyn Error>> {
    let input_string = input.to_string();
    let mut output = Vec::with_capacity(input_string.len());
    for char in input_string.chars() {
        output.push(char.to_digit(10).ok_or("Invalid char")? as i16);
    }

    Ok(output)
}

struct BasePatternIterator {
    i: usize,
    rep: usize,
}

impl BasePatternIterator {
    pub fn new(rep: usize) -> Self {
        BasePatternIterator { i: 1, rep }
    }
}

impl Iterator for BasePatternIterator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.i / self.rep) % 4;

        self.i += 1;
        Some(BASE_PATTERN[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_base_pattern_iterator(i: usize, expected: Vec<i16>) {
        let out: Vec<_> = BasePatternIterator::new(i).take(8).collect();
        assert_eq!(out, expected);
    }

    fn progress(input: &Vec<i16>) -> Vec<i16> {
        let matrix = calculate_progress_matrix(input.len());
        progress_with_matrix(&input, &matrix)
    }

    #[test]
    fn base_pattern_iterator_1() {
        test_base_pattern_iterator(1, vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn base_pattern_iterator_2() {
        test_base_pattern_iterator(2, vec![0, 1, 1, 0, 0, -1, -1, 0]);
    }

    #[test]
    fn base_pattern_iterator_3() {
        test_base_pattern_iterator(3, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }

    #[test]
    fn progress_12345678() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected_result = vec![4, 8, 2, 2, 6, 1, 5, 8];

        let result = progress(&input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn progress_48226158() {
        let input = vec![4, 8, 2, 2, 6, 1, 5, 8];
        let expected_result = vec![3, 4, 0, 4, 0, 4, 3, 8];

        let result = progress(&input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn progress_34040438() {
        let input = vec![3, 4, 0, 4, 0, 4, 3, 8];
        let expected_result = vec![0, 3, 4, 1, 5, 5, 1, 8];

        let result = progress(&input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn progress_03415518() {
        let input = vec![0, 3, 4, 1, 5, 5, 1, 8];
        let expected_result = vec![0, 1, 0, 2, 9, 4, 9, 8];

        let result = progress(&input);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn to_i16_array_03415518() {
        let input = "03415518";
        let expected_result = vec![0, 3, 4, 1, 5, 5, 1, 8];

        let result = to_i16_array(input).expect("Should not fail");
        assert_eq!(result, expected_result);
    }

    fn test_progress_n_100(value: &str, expected_8: Vec<i16>) {
        let input = to_i16_array(value).expect("Should not fail");
        let expected_result = expected_8;

        let result: Vec<_> = progress_n(input, 100).into_iter().take(8).collect();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn progress_n_80871224585914546619083218645595() {
        test_progress_n_100(
            "80871224585914546619083218645595",
            vec![2, 4, 1, 7, 6, 1, 7, 6],
        );
    }

    #[test]
    fn progress_n_19617804207202209144916044189917() {
        test_progress_n_100(
            "19617804207202209144916044189917",
            vec![7, 3, 7, 4, 5, 4, 1, 8],
        );
    }

    #[test]
    fn progress_n_69317163492948606335995924319873() {
        test_progress_n_100(
            "69317163492948606335995924319873",
            vec![5, 2, 4, 3, 2, 1, 3, 3],
        );
    }

    #[test]
    fn test_calculate_progress_matrix() {
        let expected_result = vec![
            vec![1, 0, -1, 0, 1, 0, -1, 0],
            vec![0, 1, 1, 0, 0, -1, -1, 0],
            vec![0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 1, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
        ];

        let result = calculate_progress_matrix(8);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn part_2_03036732577212944063491565474664() {
        let input = to_i16_array("03036732577212944063491565474664").expect("Should not fail");

        let result: Vec<_> = progress_n(input, 10000).into_iter().take(8).collect();
        assert_eq!(result, vec![]);
    }
}
