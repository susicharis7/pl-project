use std::io::{self, Write};

use crate::game::rules::{Gesture, Ruleset};

/* Reads Input */
pub fn read_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()  
}


/* Reads Input & Turns them into Lowercase (for commands, gestures, y/n answers) */
pub fn read_input_lowercase(prompt: &str) -> String {
    read_input(prompt).to_lowercase()
}

pub fn read_nonempty(prompt: &str) -> String {
    loop {
        let input = read_input(prompt);

        if !input.trim().is_empty() {
            return input;
        }

        println!("Input cannot be empty. Please try again.");
    }
}

pub fn read_number(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);  

        match input.parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

pub fn parse_gesture(input: &str, ruleset: &Ruleset) -> Option<Gesture> {
    match input.to_lowercase().as_str() { 
        "r" | "rock" => Some(Gesture::Rock),
        "p" | "paper" => Some(Gesture::Paper),
        "s" | "scissors" => Some(Gesture::Scissors),
        // Extended only:
        "l" | "lizard" if matches!(ruleset, Ruleset::Extended) => Some(Gesture::Lizard),
        // spec: short form `k` for Spock plus full word
        "k" | "spock" if matches!(ruleset, Ruleset::Extended) => Some(Gesture::Spock),
        _ => None,
    }
}

pub fn read_gesture(prompt: &str, ruleset: &Ruleset) -> Gesture {
    loop {
        let input = read_input(prompt);  

        // parse_gesture() does lowercase `internally`
        if let Some(g) = parse_gesture(&input, ruleset) {  
            return g;
        }

        println!("Invalid gesture. Valid:");
        match ruleset {
            Ruleset::Classic => println!("r/rock, p/paper, s/scissors"),
            Ruleset::Extended => {
                println!("r/rock, p/paper, s/scissors, l/lizard, k/spock");
            }
        }
    }
}