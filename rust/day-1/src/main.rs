use std::arch::aarch64::uint64x1_t;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::num::ParseIntError;
use std::path::Path;
use std::time::Instant;

fn main() {
    let stopwatch = Instant::now();
    let result = process_input("./src/input.txt".to_string());
    println!("Calibration value: {}", result.value);

    let mut times: Vec<u64> = Vec::new();
    for run in 1..11 {
        let benchmark_stopwatch = Instant::now();
        let result = process_input("./src/input.txt".to_string());
        times.push(benchmark_stopwatch.elapsed().as_millis() as u64)
    }
    println!("Official average input: {}ms", times.iter().sum::<u64>() as f64 / times.len() as f64);

    for run in 1..3 {
        let large_stopwatch = Instant::now();
        let large_result = process_input("./src/large-input.txt".to_string());
        println!("Calibration value: {}", large_result.value);
        println!("Execution time run {}: {}ms", run, large_stopwatch.elapsed().as_millis());
    }

}

fn process_input(filename: String) -> CalibrationNumber {
    return read_lines(filename)
        .unwrap()
        .map(|line| parse_calibration_digit(line.unwrap()))
        .map(|digit| create_calibration_number(digit))
        .fold(
            CalibrationNumber { value: 0 },
            |acc, number| CalibrationNumber { value: acc.value + number.value },
        );
}

fn parse_calibration_digit(line: String) -> CalibrationDigit {
    let characters = line.split("").collect::<Vec<&str>>();

    let mut digits: Vec<u64> = Vec::new();
    let mut processed_characters = "".to_owned();
    for character in characters.iter().copied() {
        match character.parse::<u64>() {
            Ok(digit) => { digits.push(digit) }
            Err(non_digit) => {}
        }
        processed_characters.push_str(character.clone());

        if processed_characters.ends_with("one") {
            digits.push(1)
        } else if processed_characters.ends_with("two") {
            digits.push(2)
        } else if processed_characters.ends_with("three") {
            digits.push(3)
        } else if processed_characters.ends_with("four") {
            digits.push(4)
        } else if processed_characters.ends_with("five") {
            digits.push(5)
        } else if processed_characters.ends_with("six") {
            digits.push(6)
        } else if processed_characters.ends_with("seven") {
            digits.push(7)
        } else if processed_characters.ends_with("eight") {
            digits.push(8)
        } else if processed_characters.ends_with("nine") {
            digits.push(9)
        }
    }

    return CalibrationDigit {
        first_digit: digits[0],
        second_digit: digits[digits.len() - 1],
    };
}

fn create_calibration_number(digit: CalibrationDigit) -> CalibrationNumber {
    let concat_digits = digit.first_digit.to_string() + &digit.second_digit.to_string();
    return CalibrationNumber {
        value: concat_digits.parse::<u64>().unwrap()
    };
}

fn sum_calibration_digits(numbers: Vec<CalibrationNumber>) -> CalibrationNumber {
    return numbers
        .iter()
        .fold(
            CalibrationNumber { value: 0 },
            |acc, number| CalibrationNumber { value: acc.value + number.value },
        );
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[derive(PartialEq, Debug)]
struct CalibrationDigit {
    first_digit: u64,
    second_digit: u64,
}

#[derive(PartialEq, Debug)]
struct CalibrationNumber {
    value: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_single_line_with_two_digits() {
        let actual = parse_calibration_digit("1abc2".to_string());

        assert_eq!(
            actual,
            CalibrationDigit {
                first_digit: 1,
                second_digit: 2,
            }
        )
    }

    #[test]
    fn test_parse_single_line_with_one_digit() {
        let actual = parse_calibration_digit("a7bc".to_string());

        assert_eq!(
            actual,
            CalibrationDigit {
                first_digit: 7,
                second_digit: 7,
            }
        )
    }

    #[test]
    fn test_parse_line_wth_multiple_digits() {
        let actual = parse_calibration_digit("a1b2c3d4e5f".to_string());
        assert_eq!(
            actual,
            CalibrationDigit {
                first_digit: 1,
                second_digit: 5,
            }
        )
    }

    #[test]
    fn test_parse_line_with_spelt_one() {
        let actual = parse_calibration_digit("one".to_string());
        assert_eq!(
            actual,
            CalibrationDigit {
                first_digit: 1,
                second_digit: 1,
            }
        )
    }

    #[test]
    fn test_create_calibration_number() {
        let actual = create_calibration_number(CalibrationDigit {
            first_digit: 1,
            second_digit: 9,
        });

        assert_eq!(
            actual,
            CalibrationNumber { value: 19 }
        )
    }

    #[test]
    fn test_process_test_input() {
        let actual = process_input("./src/test-input.txt".to_string());
        assert_eq!(
            actual,
            CalibrationNumber { value: 142 }
        )
    }

    #[test]
    fn test_process_test_input_part_two() {
        let actual = process_input("./src/test-input-part-2.txt".to_string());
        assert_eq!(
            actual,
            CalibrationNumber { value: 299 }
        )
    }
}