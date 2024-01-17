use std::char;
use std::io;

use crate::libs::file_reader::read_lines;

struct LineTriple<'a>{
    prev: &'a String,
    curr: &'a String,
    next: Option<&'a io::Result<String>>
}

struct SubStrMark {
    index: usize,
    length: usize,
}

pub fn q1() {
    // File must exist in the current path
    if let Ok(iter) = read_lines("./src/days/day3/day3.input") {
        let mut sum: u32 = 0;
        let mut prev: String = String::from("");
        let mut lines = iter.peekable();

        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(line)) = lines.next() {
            let line_tuple = (&prev, &line, lines.peek() );
            sum += parse_line_q1(line_tuple);

            prev = line.clone();
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_line_q1(line_tuple: (&String, &String, Option<&io::Result<String>>) ) -> u32{

    let line = line_tuple.1;
    let mut value: u32 = 0;

    // Collect digits
    let mut digit_index: usize = 0;
    let mut digit_buffer: Vec<char> = Vec::new();
    let chars = line.chars().enumerate();
    for (index,c) in chars {
        // Keep track of just digits separately
        if char::is_ascii_digit(&c) {
            if digit_buffer.is_empty() {
                digit_index = index;
            }
            digit_buffer.push(c);
        }

        //  Any other char means end of a number
        if (!char::is_ascii_digit(&c) && !digit_buffer.is_empty() ) ||
            (index == line.len()-1 && !digit_buffer.is_empty() ) {
            if is_valid_part_number(line_tuple, (digit_index, digit_buffer.len()) ) {
                value += parse_digit_buffer(&digit_buffer);
            }
            digit_buffer.clear();
            continue;
        }
    }

    value
}

fn is_valid_part_number(line_tuple: (&String, &String, Option<&io::Result<String>>), number_info: (usize, usize)) -> bool {

    let prev = line_tuple.0;
    let curr = line_tuple.1;
    let next_result = line_tuple.2;

    let check_range = compute_range_value(curr.len(), number_info );

    if !prev.is_empty() &&
        check_for_symbols(prev, check_range) {
        return true
    }

    if check_for_symbols(curr, check_range) {
        return true;
    }

    if let Some(Ok(next)) = next_result {
        if check_for_symbols(next, check_range) {
            return true;
        }
    }

    false
}

fn compute_range_value(len: usize, number_info: (usize, usize)) -> (usize, usize) {

    let num_start = number_info.0;
    let num_len = number_info.1;
    let num_end = num_start + num_len;

    // Check for left edge
    let start = if num_start == 0 { 0 } else { num_start - 1 };

    // Check for right edge
    let mut length = if num_end == len { num_len } else { num_len + 1 };
    // Check for left ege
    length = if num_start == 0 { length } else { length + 1 };

    (start, length)
}

fn check_for_symbols(line: &str, range_info: (usize, usize)) -> bool {
    let start = range_info.0;
    let length = range_info.1;

    let chars = line.chars().skip(start);
    let mut count: usize = 0;
    for c in chars {
        count += 1;
        print!("{} ", c);

        // Symbol Check
        if !c.is_ascii_digit() && c != '.' {
            println!();
            return true;
        }

        // Breakout condition
        if count == length {
            break;
        }
    }
    println!("|");
    false
}

fn parse_digit_buffer(buffer: &[char]) -> u32 {
    return buffer.iter().collect::<String>().parse().unwrap();
}

pub fn q2() {
    // File must exist in the current path
    if let Ok(iter) = read_lines("./src/days/day3/day3.input") {
        let mut sum: usize = 0;
        let mut prev: String = String::from("");
        let mut lines = iter.peekable();

        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(line)) = lines.next() {
            let line_triple = LineTriple { prev: &prev, curr: &line, next: lines.peek() };
            sum += parse_q2(&line_triple);
            prev = line.clone();
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_q2(line_triple: &LineTriple ) -> usize {
    let mut value: usize = 0;

    let line = line_triple.curr;
    let chars = line.chars().enumerate();
    for (index,c) in chars {
        if c == '*' {
            value += compute_gear_ratio(line_triple, SubStrMark {index, length: 1});
            continue;
        }
    }

    value
}

fn compute_gear_ratio(line_triple: &LineTriple, sub_str_mark: SubStrMark) -> usize {
    let curr = line_triple.curr;
    let mut part_numbers: Vec<usize> = Vec::new();

    let check_range = compute_range_value(curr.len(), (sub_str_mark.index, sub_str_mark.length) );
    let range = SubStrMark {index: check_range.0, length: check_range.1};

    if !line_triple.prev.is_empty() {
        let gathered = gather_part_numbers(line_triple.prev, &range);
        part_numbers.extend(gathered);
    }

    let gathered = gather_part_numbers(line_triple.curr, &range);
    part_numbers.extend(gathered);

    if let Some(Ok(next)) = line_triple.next {
        let gathered = gather_part_numbers(next, &range);
        part_numbers.extend(gathered);
    }

    if part_numbers.len() != 2 {
        0
    } else {
        part_numbers.into_iter().reduce(|acc, val| acc * val).unwrap()
    }
}

fn gather_part_numbers(line: &String, sub_str_mark: &SubStrMark) -> Vec<usize> {
    let mut part_numbers: Vec<usize> = Vec::new();

    // Collect digits
    let mut digit_index: usize = 0;
    let mut digit_buffer: Vec<char> = Vec::new();
    let chars = line.chars().enumerate();
    for (index,c) in chars {
        // Keep track of just digits separately
        if char::is_ascii_digit(&c) {
            if digit_buffer.is_empty() {
                digit_index = index;
            }
            digit_buffer.push(c);
        }

        // Any other char means end of a number
        // End of line (read_line strip newline?)
        if (!char::is_ascii_digit(&c) && !digit_buffer.is_empty() ) ||
            (index == line.len()-1 && !digit_buffer.is_empty() ) {

            let start = sub_str_mark.index;
            let end = sub_str_mark.index + sub_str_mark.length - 1;
            let number_start = digit_index;
            let number_end = digit_index + digit_buffer.len() - 1;


            // Is valid part number
            if (number_start >= start && number_start <= end) ||
                (number_end >= start && number_end <= end) {
                part_numbers.push(parse_digit_buffer(&digit_buffer) as usize);
            }

            digit_buffer.clear();
            continue;
        }
    }

    part_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_checking_symbols() {
        let mut line;

        line = String::from("....123...");
        assert_eq!(false, check_for_symbols(&line, (3, 5)), "Mid, No symbols.");

        line = String::from("..%.123.&.");
        assert_eq!(false, check_for_symbols(&line, (3, 5)), "Mid, Symbols too far.");

        line = String::from(".....&....");
        assert_eq!(true, check_for_symbols(&line, (3, 5)), "Other, Symbol center.");
    }

    #[test]
    fn is_checking_left_edge() {
        let mut line;

        line = String::from("...%123...");
        assert_eq!((3,5), compute_range_value(10, (4, 3)), "Range left.");
        assert_eq!(true, check_for_symbols(&line, (3, 5)), "Symbol left.");

        line = String::from("...%......");
        assert_eq!((3,5), compute_range_value(10, (4, 3)), "Other, Range left.");
        assert_eq!(true, check_for_symbols(&line, (3, 5)), "Other, Symbol left.");

        line = String::from("*.........");
        assert_eq!((0,5), compute_range_value(10, (1, 3)), "Range left edge.");
        assert_eq!(true, check_for_symbols(&line, (0, 5)), "Symbol left edge.");

        line = String::from("123*......");
        assert_eq!((0,4), compute_range_value(10, (0, 3)), "Range left edge inner.");
        assert_eq!(true, check_for_symbols(&line, (0, 4)), "Symbol left edge inner.");
    }

    #[test]
    fn is_checking_middle() {
        let mut line;

        line = String::from(".....2)...");
        assert_eq!(true, check_for_symbols(&line, (4, 3)), "Mid, 1 Digit.");
    }

    #[test]
    fn is_checking_right_edge() {
        let mut line;

        line = String::from("....123@..");
        assert_eq!((3,5), compute_range_value(10, (4, 3)), "Range right.");
        assert_eq!(true, check_for_symbols(&line, (3, 5)), "Symbol right.");

        line = String::from(".......@..");
        assert_eq!((3,5), compute_range_value(10, (4, 3)), "Other, Range right.");
        assert_eq!(true, check_for_symbols(&line, (3, 5)), "Other, Symbol right.");

        line = String::from(".........^");
        assert_eq!((5,5), compute_range_value(10, (6, 3)), "Range right edge.");
        assert_eq!(true, check_for_symbols(&line, (5, 5)), "Symbol right edge.");

        line = String::from("......^123");
        assert_eq!((6,4), compute_range_value(10, (7, 3)), "Range right edge inner.");
        assert_eq!(true, check_for_symbols(&line, (6, 4)), "Symbol right edge inner.");
    }
}
