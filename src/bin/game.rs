use std::io::stdin;

use colored::*;

use fhcli::util::env::{get_word_base, get_app_language, get_max_guesses};
use fhcli::util::lang::replace_unicode;

fn main() -> std::io::Result<()> {
    let max_guesses: i32 = get_max_guesses();

    let word_base = get_word_base().unwrap();

    match word_base.get_random_word() {
        None => println!("¯\\_({})_/¯ Seems like I ran out of words! Have you tried using the import tool?", "\u{30c4}"),
        Some(solution_entry) => {
            let solution: &str = &solution_entry.word.to_lowercase();

            print_welcome();

            println!("Welcome to fancy hangman CLI! Guess today's word!");

            solution.chars().into_iter().for_each(| _ | print!("{} ", "_"));
            println!();

            println!("You have {} guesses.", max_guesses);

            let mut full_match: bool = false;

            let mut counter = 0;
            while counter < max_guesses {
                let input: String = read_input(solution.len());
                let guess: String = input.to_lowercase();

                match word_base.find_word(&guess) {
                    Some(_) => {
                        let guesses: i32 = max_guesses - counter - 1;
                        full_match = check_word(&solution, &guess);

                        if full_match == true {
                            break;
                        } else {
                            if guesses > 1 {
                                println!("You now have {} guesses.", guesses);
                            } else {
                                println!("This is your last guess.");
                            }
                        }

                        if guesses == 0 { println!("Better luck next time!") }

                        counter += 1;
                    },
                    None => println!("The guessed word is not in the word list.")
                }
            }

            if full_match == true {
                println!("Congratulations! You won!");
                word_base.update_word(solution_entry);
            }
        }
    }

    Ok(())
}

/// Read stdin user input and validate against the expected word length.
/// Read until valid input has been entered.
///
/// # Arguments
///
/// - `word_len` - The expected word length
fn read_input(word_len: usize) -> String {
    let mut input: String = String::new();

    loop {
        stdin().read_line(&mut input).unwrap();
        let polished = replace_unicode(&input, get_app_language());

        let trim: &str = polished.trim();

        if !validate_user_input(trim, word_len) {
            println!("Invalid input: Your guess must have a size of {} characters. You entered {} characters.", word_len, trim.len());

            input = String::new();
        } else {
            input = trim.to_lowercase();

            break;
        }
    }

    input
}

fn validate_user_input(user_input: &str, expected_len: usize) -> bool {
    user_input.len() == expected_len
}

/// Check guessed word against solution and color each letter accordingly by comparing the respective
/// character index. A full match results in an instant win.
///
/// * `green color` means that a letter's position has been guessed correctly
/// * `yellow color` means that a letter is in the word, but at another position.
fn check_word(solution_word: &str, guessed_word: &str) -> bool {
    let guessed_characters: Vec<char> = guessed_word.chars().collect();
    let solution_characters: Vec<char> = solution_word.chars().collect();

    for i in 0..guessed_word.len() {
        let index: Option<usize> = solution_word.find(guessed_characters[i]);

        match index {
            Some(_index) => {
                if solution_characters[i] == guessed_characters[i] {
                    print!("{} ", guessed_characters[i].to_string().color("green"))
                } else {
                    print!("{} ", guessed_characters[i].to_string().color("yellow"))
                }
            }
            None => { print!("{} ", guessed_characters[i]) }
        }
    }

    println!();

    // check for full match
    if String::from(solution_word).to_lowercase().eq(guessed_word) {
        return true;
    }

    false
}

fn print_welcome() {
    println!(r#"
 _______  __    __    ______  __       __         .______          _______.
|   ____||  |  |  |  /      ||  |     |  |        |   _  \        /       |
|  |__   |  |__|  | |  ,----'|  |     |  |  ______|  |_)  |      |   (----`
|   __|  |   __   | |  |     |  |     |  | |______|      /        \   \
|  |     |  |  |  | |  `----.|  `----.|  |        |  |\  \----.----)   |
|__|     |__|  |__|  \______||_______||__|        | _| `._____|_______/
    "#)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_check_word() {
        use super::check_word;

        assert_eq!(check_word("mario", "mario"), true);
        assert_ne!(check_word("mario", "wario"), true);
    }

    #[test]
    fn test_validate_user_input() {
        use super::validate_user_input;

        assert_eq!(validate_user_input("mario" , 5), true);
        assert_eq!(validate_user_input("apfelsaft", 9), true);
        assert_eq!(validate_user_input("lüge", 5), true);

        assert_ne!(validate_user_input("lüge", 4), true);
        assert_ne!(validate_user_input("luege", 4), true);
    }
}
