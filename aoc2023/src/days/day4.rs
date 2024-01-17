use std::char;

use crate::libs::file_reader::read_lines;

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
    #[derive(PartialEq)]
    enum LineSection {
        Info,
        Winners,
        Picks,
    }

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

// pub fn q2() {
//
// }

// fn parse_q2(line_triple: &LineTriple ) -> usize {
//
// }

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

}


