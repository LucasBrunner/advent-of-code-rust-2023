use lazy_static::lazy_static;
use regex::Regex;

pub fn part_1(input: &str) {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digit_1 = line
                .chars()
                .find(|c| c.is_digit(10))
                .expect("Line did not contain any digits!");

            let digit_2 = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .expect("Line did not contain any digits!");

            format!("{}{}", digit_1, digit_2).parse::<u32>().unwrap()
        })
        .sum();

    println!("The sum of all calibration values is {}", sum);
}

fn parse_digit_from_word(word: &str) -> u8 {
    match word.to_lowercase().as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("word was not a valid digit word"),
    }
}

lazy_static! {
    pub static ref FORWARD_DIGIT_REGEX: Regex =
        Regex::new(r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    pub static ref BACKWARD_DIGIT_REGEX: Regex =
        Regex::new(r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)").unwrap();
}

fn replace_first_digit_forwards(line: &str) -> String {
    let forwards_replace = FORWARD_DIGIT_REGEX.find(line).map(|found| found.as_str());
    let output = match forwards_replace {
        Some(found) => FORWARD_DIGIT_REGEX
            .replace(line, parse_digit_from_word(found).to_string())
            .to_string(),
        None => line.to_owned(),
    };

    output
}

fn replace_first_digit_backwords(line: &str) -> String {
    let line_reverse = line.chars().rev().collect::<String>();
    let backwords_replace = &BACKWARD_DIGIT_REGEX
        .find(&line_reverse)
        .map(|found| found.as_str().chars().rev().collect::<String>());

    let output = match backwords_replace {
        Some(found) => BACKWARD_DIGIT_REGEX
            .replace(&line_reverse, parse_digit_from_word(found).to_string())
            .to_string(),
        None => line_reverse,
    };

    output.chars().rev().collect()
}

fn part_2(input: &str) {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digit_1 = replace_first_digit_forwards(line)
                .chars()
                .find(|c| c.is_digit(10))
                .expect("Line did not contain any digits!");

            let digit_2 = replace_first_digit_backwords(line)
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .expect("Line did not contain any digits!");

            format!("{}{}", digit_1, digit_2).parse::<u32>().unwrap()
        })
        .sum();

    println!(
        "The sum of all calibration values with spelled out letters is {}",
        sum
    );
}

pub fn run() {
    let input = include_str!("../inputs/day_01.txt");

    part_1(input);
    part_2(input);
}
