use std::char;
use std::collections::HashMap;

use crate::libs::file_reader::read_lines;

#[derive(PartialEq)]
enum LineSection {
    Info,
    Winners,
    Picks,
}

pub fn q1() {
    // File must exist in the current path
    if let Ok(mut iter) = read_lines("./src/days/day4.input") {
        let mut sum: u32 = 0;

        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(line)) = iter.next() {
            sum += parse_line_q1(line);
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_line_q1(line: String) -> u32{

    let mut winners: Vec<u8> = Vec::new();
    let mut picks: Vec<u8> = Vec::new();
    let mut section: LineSection = LineSection::Info;

    let mut digit_buffer: Vec<char> = Vec::new();
    let chars = line.chars().enumerate();
    for (index,c) in chars {

        // Check for section update
        if c == ':' {
            section = LineSection::Winners;
            continue;
        } else if c == '|' {
            section = LineSection::Picks;
            continue;
        }

        // No need to parse
        if section == LineSection::Info {
            continue;
        }

        // Keep track of the digits
        if char::is_ascii_digit(&c) {
            digit_buffer.push(c);
        }

        if (!char::is_ascii_digit(&c) && !digit_buffer.is_empty() ) ||
            (index == line.len()-1 && !digit_buffer.is_empty() ) {
            let num = parse_digit_buffer(&digit_buffer);

            if section == LineSection::Winners {
                winners.push(num);
            } else if section == LineSection::Picks {
                picks.push(num);
            }
            digit_buffer.clear();
        }
    }

    calculate_points(&winners, &picks)
}

fn parse_digit_buffer(buffer: &[char]) -> u8 {
    let string: String = buffer.iter().collect();
    string.parse().unwrap()
}

fn calculate_points(winners: &[u8], picks: &[u8]) -> u32 {
    let mut points: u32 = 0;

    for value in winners {
        if picks.iter().any(|&x| x == *value) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }

    points
}

pub fn q2() {
    // File must exist in the current path
    if let Ok(mut iter) = read_lines("./src/days/day4.input") {
        let mut sum: u32 = 0;
        let mut carry_forward: HashMap<u8, u32> = HashMap::new();

        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(line)) = iter.next() {
            let result = parse_q2(&line, &mut carry_forward);
            sum += result.0;
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_q2<'a>(line: &str, card_copies: &'a mut HashMap<u8, u32>) -> (u32, &'a HashMap<u8, u32>) {
    let mut card_number: u8 = 0;
    let mut winners: Vec<u8> = Vec::new();
    let mut picks: Vec<u8> = Vec::new();
    let mut section: LineSection = LineSection::Info;

    let mut digit_buffer: Vec<char> = Vec::new();
    let chars = line.chars().enumerate();
    for (index,c) in chars {

        if char::is_ascii_digit(&c) {
            digit_buffer.push(c);
        }

        if (!char::is_ascii_digit(&c) && !digit_buffer.is_empty() ) ||
            (index == line.len()-1 && !digit_buffer.is_empty() ) {
            let num = parse_digit_buffer(&digit_buffer);

            if section == LineSection::Winners {
                winners.push(num);
            } else if section == LineSection::Picks {
                picks.push(num);
            } else {
                card_number = num;
            }
            digit_buffer.clear();
        }

        // Check for section update
        if c == ':' {
            section = LineSection::Winners;
        } else if c == '|' {
            section = LineSection::Picks;
        }
    }
    // Add card copies based on matches
    let card_count = 1 + card_copies.remove(&card_number).unwrap_or_default();
    // println!("Card {}: {} copies", card_number, card_count-1);
    let matches = calculate_matches(&winners, &picks);
    if matches > 0 {
        for i in 1..(matches+1) {
            let target_card_number = card_number + i;
            // print!("{}, ", target_card_number);
            card_copies.entry(target_card_number)
                .and_modify(|copies| *copies += card_count)
                .or_insert(card_count);
        }
        // println!();
    }

    (card_count, card_copies)
}

fn calculate_matches(winners: &[u8], picks: &[u8]) -> u8 {
    let mut matches: u8 = 0;

    for value in winners {
        if picks.iter().any(|&x| x == *value) {
           matches += 1;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_calculating_points() {
        let winners: Vec<u8> = vec![41, 48, 83, 86, 17];
        let picks: Vec<u8> = vec![83, 86, 6, 31, 17, 9, 48, 53];

        let points = calculate_points(&winners, &picks);
        assert_eq!(8, points);
    }

    #[test]
    fn is_parsing_line() {
        let mut line;

        line = String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(8, parse_line_q1(line), "Multiple matches");

        line = String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        assert_eq!(1, parse_line_q1(line), "Single match");

        line = String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        assert_eq!(0, parse_line_q1(line), "No matches");

        line = String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(2, parse_line_q1(line), "EOL match");
    }

    #[test]
    fn is_calculating_matches() {
        let winners: Vec<u8> = vec![41, 48, 83, 86, 17];
        let picks: Vec<u8> = vec![83, 86, 6, 31, 17, 9, 48, 53];

        let matches = calculate_matches(&winners, &picks);
        assert_eq!(4, matches);
    }

    #[test]
    fn is_parsing_line_q2() {
        let mut line;
        let mut carry_forward: HashMap<u8, u32>;

        carry_forward = HashMap::new();
        line = String::from("Card   1: 58 68  1 21 88 37 66 61 23 25 | 63 95 45 43 79 64 29 87  8 70 84 34 91 67  3 76 27 24 28 62 13 54 19 93  7");
        let result = parse_q2(&line, &mut carry_forward);
        assert_eq!(1, result.0, "No matches");
        assert_eq!(true, result.1.is_empty(), "No matches");
    }

}


