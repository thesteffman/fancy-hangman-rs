# fancy-hangman-rs
fancy-hangman-rs is a wordle inspired word guessing game for the CLI written in rust.

## Usage
Run the game by executing
`
cargo run --bin game
`

Run the importer tool by executing
`
cargo run --bin import <file_path> <locale>
`

## Game rules
The player has to correctly guess a randomly selected word from the word base. All words are 5 characters long. By coloring single letters the game tells the player about correct letter positioning.
* green: The the guessed letter is at the correct position.
* orange: The word contains the letter, but at a different position.

The game ends when the player runs out of guesses or when the player guesses the word correctly. After that, a message is displayed. 

## Settings
The `.env` file contains various settings to adjust the game's behavior.
* `MAX_GUESSES` indicates how many guesses the player may give to win.
* `LOCALE` indicates the language of the user input. If unset, it defaults to ``en``.
* `DATABASE_URL` indicates that the word base is located in a given postgres db
* `WORDBASE_FILE` indicates that the word base is located in a given text file

