use super::{
    io::programmable::{ProgrammableInput, ProgrammableOutput},
    program::Program,
};
use std::borrow::Cow;

pub struct Circuit {
    program: Vec<i32>,
}

impl Circuit {
    pub fn new(program: Vec<i32>) -> Self {
        Self { program }
    }

    pub fn run(&self, phase_settings_sequence: &[i32]) -> Result<i32, Cow<'static, str>> {
        if phase_settings_sequence.len() < 2 {
            return Err("There should be at least 2 items in the sequence".into());
        } else {
            let mut last_output = 0;

            for (phase, phase_setting) in phase_settings_sequence.iter().enumerate() {
                let mut phase_input = ProgrammableInput::new(vec![*phase_setting, last_output]);
                let mut output = ProgrammableOutput::new();

                {
                    let mut amplifier =
                        Program::new(self.program.clone(), &mut phase_input, &mut output);
                    amplifier.run()?;
                }

                let output_as_vec = output.output();

                last_output = *output_as_vec
                    .first()
                    .ok_or(format!("Phase {} did not return any output", phase))?;
            }

            Ok(last_output)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_day_07_part1_example(program: Vec<i32>, phase_setting: &[i32], expected_result: i32) {
        let circuit = Circuit::new(program);
        let result = circuit.run(phase_setting);

        assert_eq!(result, Ok(expected_result));
    }

    #[test]
    fn day_07_part1_example1() {
        run_day_07_part1_example(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &[4, 3, 2, 1, 0],
            43210,
        );
    }

    #[test]
    fn day_07_part1_example2() {
        run_day_07_part1_example(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            &[0, 1, 2, 3, 4],
            54321,
        );
    }

    #[test]
    fn day_07_part1_example3() {
        run_day_07_part1_example(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            &[1, 0, 4, 3, 2],
            65210,
        );
    }
}
