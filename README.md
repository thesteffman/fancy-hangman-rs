# wordle-cli
![crates.io](https://img.shields.io/crates/v/wordle-cli.svg)
[![.github/workflows/build.yml][build-badge]][build-url]
[![.github/workflows/test.yml][test-badge]][test-url]

[build-badge]: https://github.com/tohuwabohu-io/wordle-cli/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/tohuwabohu-io/wordle-cli/actions/workflows/build.yml
[test-badge]: https://github.com/tohuwabohu-io/wordle-cli/actions/workflows/test.yml/badge.svg
[test-url]: https://github.com/tohuwabohu-io/wordle-cli/actions/workflows/test.yml

wordle-cli is a wordle inspired word guessing game for the CLI written in rust.

## Usage
Run the game by executing
`
cargo run --bin game [language]
`.

If `language` is not set, it defaults to the locale system settings or `"en"`.


Run the import tool by executing
`
cargo run --bin import <file_path> [language]
`.

If `language` is not set, it defaults to the locale system settings or `"en"`.

## Game rules
The player has to correctly guess a randomly selected word from the dictionary. All words are 5 characters long. By coloring single letters the game tells the player about correct letter positioning.
* green: The guessed letter is at the correct position.
* orange: The word contains the letter, but at a different position.

The game ends when the player runs out of guesses or when the player guesses the word correctly. After that, a message is displayed. 

## Settings
The `.env` file contains information about the database location.
* `DATABASE_URL` indicates that the dictioanry is located in a given db url

## Import
The import tool can be used to expand the word base. Usage: See Usage above.

However, the requirements for the file underneath the `<file_path>` argument are as follows:
* The file needs to be encoded in UTF-8
* The words need to be separated with a newline character as the file is read line-wise

The tool automatically removes duplicates and entries with a size different from 5 characters and converts unicode characters to ASCII using [any_ascii]. German umlauts receive a special treatment.

## Database
If you choose to set up a database to serve as dictionary, please take a closer look at the [diesel.rs] documentation. You need a working diesel_cli installation to proceed.

Step 1: Run the diesel initialization.

`diesel setup`

Step 2: Create a diesel migration.

`diesel create dictionary_migration`

Step 3: Navigate to the res/db folder.

`create.sql` and `drop.sql` should be put into the diesel migration's `up.sql` and `down.sql`.

Step 4: Import your dictionary into the database. 

`cargo run --bin import dictionary.txt [language]`

Et voilà! Enjoy additional features. A correctly guessed word will be marked as `guessed` in the database and won't show up a second time. The first randomly selected word of the day will re-occur upon starting the game on the same day, until it has been guessed successfully.

[diesel.rs]: http://diesel.rs/guides/getting-started 
[any_ascii]: https://github.com/anyascii/anyascii
