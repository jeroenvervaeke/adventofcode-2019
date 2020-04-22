mod circuit;
mod io;
mod operations;
mod program;

use circuit::Circuit;
use operations::Operation;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_07/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let int_code = line
        .trim()
        .split(',')
        .map(|digit_str| digit_str.parse::<i32>().expect("Invalid input file"))
        .collect();

    let circuit = Circuit::new(int_code);

    let mut max_value = None;

    for phase_0 in 0..5 {
        for phase_1 in 0..5 {
            for phase_2 in 0..5 {
                for phase_3 in 0..5 {
                    for phase_4 in 0..5 {
                        let phase_sequence = &[phase_0, phase_1, phase_2, phase_3, phase_4];
                        let unique_phase_sequences: HashSet<&i32> = phase_sequence.iter().collect();
                        if phase_sequence.len() != unique_phase_sequences.len() {
                            continue;
                        }

                        let value = circuit.run(phase_sequence)?;

                        match max_value {
                            Some(max) if max < value => {
                                max_value = Some(value);
                            }
                            None => {
                                max_value = Some(value);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    println!("value: {:?}", max_value);

    Ok(())
}
