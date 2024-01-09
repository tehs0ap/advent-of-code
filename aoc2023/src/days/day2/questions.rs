use std::char;

use crate::libs::file_reader::read_lines;


pub fn q1() {

    let mut sum: u32 = 0;

    // File must exist in the current path
    if let Ok(mut lines) = read_lines("./src/days/day2/day2.input") {
        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(line)) = lines.next() {
            sum += parse_line_q1(&line);                
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_line_q1(line: &String) -> u32{

    let mut game_id: u32 = 0;
    let mut blue: u32 = 0;
    let mut green: u32 = 0;
    let mut red: u32 = 0;

    let mut digit_buffer: Vec<char> = Vec::new(); 
    let mut char_buffer: Vec<char> = Vec::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        // Keep track of just digits separately
        if char::is_ascii_digit(&c) {
            digit_buffer.push(c.clone());
            continue;
        }
        
        // Parse the game id
        if c == ':' {
            game_id = parse_digit_buffer(&digit_buffer);
            digit_buffer.clear();
            char_buffer.clear();
            continue;
        }

        if (c == 'b' || c == 'g' || c == 'r') && 
            char_buffer.last().is_some_and(|x: &char| *x == ' ') { 
            
            let num = parse_digit_buffer(&digit_buffer);
            
            if c == 'b' && blue < num {
                blue = num;
            } else if c == 'g' && green < num {
                green = num;
            } else if c == 'r' && red < num {
                red = num;
            }
            
            digit_buffer.clear();
            char_buffer.clear();
            continue;
        }

        char_buffer.push(c.clone());
    }
    
    if blue > 14 || green > 13 || red > 12 {
        game_id = 0;
    }

    return game_id;
}

fn parse_digit_buffer(buffer: &Vec<char>) -> u32 {
    return buffer.into_iter().collect::<String>().parse().unwrap();
}

pub fn q2() {

    let mut sum: u32 = 0;

    // File must exist in the current path
    if let Ok(mut lines) = read_lines("./src/days/day2/day2.input") {
        // Consumes the iterator, returns an (Optional) String
        while let Some(Ok(line)) = lines.next() {
            sum += parse_line_q2(&line);                
        }
        println!("{}",sum);
    } else {
        println!("Could not read file!")
    }
}

fn parse_line_q2(line: &String) -> u32{

    let mut game_id: u32 = 0;
    let mut blue: u32 = 0;
    let mut green: u32 = 0;
    let mut red: u32 = 0;

    let mut digit_buffer: Vec<char> = Vec::new(); 
    let mut char_buffer: Vec<char> = Vec::new();
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        // Keep track of just digits separately
        if char::is_ascii_digit(&c) {
            digit_buffer.push(c.clone());
            continue;
        }
        
        // Parse the game id
        if c == ':' {
            game_id = parse_digit_buffer(&digit_buffer);
            digit_buffer.clear();
            char_buffer.clear();
            continue;
        }

        if (c == 'b' || c == 'g' || c == 'r') && 
            char_buffer.last().is_some_and(|x: &char| *x == ' ') { 
            
            let num = parse_digit_buffer(&digit_buffer);
            
            if c == 'b' && blue < num {
                blue = num;
            } else if c == 'g' && green < num {
                green = num;
            } else if c == 'r' && red < num {
                red = num;
            }
            
            digit_buffer.clear();
            char_buffer.clear();
            continue;
        }

        char_buffer.push(c.clone());
    }
    
    return blue * green * red;
}
