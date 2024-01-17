use std::{char, usize};
use crate::libs::file_reader::read_lines;

pub fn q1() {

    let mut sum: u32 = 0;

    // File must exist in the current path
    if let Ok(mut lines) = read_lines("./src/days/day1/day1.input") {
        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(calibration)) = lines.next() {
            let mut index = calibration.find( |c: char| char::is_digit(c, 10) ).unwrap();
            let first_digit = char::from( calibration.as_bytes()[index] );

            index = calibration.rfind( |c: char| char::is_digit(c, 10) ).unwrap();
            let last_digit = char::from( calibration.as_bytes()[index] );

            let value_str = first_digit.to_string() + &last_digit.to_string();
            let value: u32 = value_str.parse().unwrap();
            sum += value;
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }

}

pub fn q2() {

    let mut sum: u32 = 0;

    // File must exist in the current path
    if let Ok(mut lines) = read_lines("./src/days/day1/day1.input") {
        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(calibration)) = lines.next() {
            sum += parse_calibration_values(&calibration);                
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_calibration_values(line: &str) -> u32 {
    
    let needles: &[_] = &[
        ("0", "0"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut first: Option<(usize, &str)> = None;
    let mut last: Option<(usize, &str)> = None;

    for needles in needles {
       let matched_indices: Vec<_> = line.match_indices(needles.0).collect();
       
       if matched_indices.is_empty() {
           continue;
       }

       if first.is_none() || matched_indices.first().unwrap().0 < first.unwrap().0 {
            first = matched_indices.first().copied();
       }

       if last.is_none() || matched_indices.last().unwrap().0 > last.unwrap().0 {
           last = matched_indices.last().copied();
       }
    }

    let first_digit = needles.iter().find(|tuple| tuple.0 == first.unwrap().1).unwrap().1;
    let second_digit = needles.iter().find(|tuple| tuple.0 == last.unwrap().1).unwrap().1;

    (first_digit.to_owned() + second_digit).parse().unwrap()
}

