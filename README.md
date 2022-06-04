# fancy-hangman-rs
[![.github/workflows/build.yml][build-badge]][build-url]
[![.github/workflows/test.yml][test-badge]][test-url]

[build-badge]: https://github.com/thesteffman/fancy-hangman-rs/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/thesteffman/fancy-hangman-rs/actions/workflows/build.yml
[test-badge]: https://github.com/thesteffman/fancy-hangman-rs/actions/workflows/test.yml/badge.svg
[test-url]: https://github.com/thesteffman/fancy-hangman-rs/actions/workflows/test.yml

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
* green: The guessed letter is at the correct position.
* orange: The word contains the letter, but at a different position.

The game ends when the player runs out of guesses or when the player guesses the word correctly. After that, a message is displayed. 

## Settings
The `.env` file contains various settings to adjust the game's behavior.
* `MAX_GUESSES` indicates how many guesses the player may give to win.
* `LOCALE` indicates the language of the user input. If unset, it defaults to ``en``.
* `DATABASE_URL` indicates that the word base is located in a given postgres db
* `WORDBASE_FILE` indicates that the word base is located in a given text file

## Import
The import tool can be used to expand the word base. You have two options here with different implications.

If `WORDBASE_FILE` in the `.env` file is set, the tool will merge the data to avoid duplicates.

If `DATABASE_URL` is set, you need to manually clean up the database beforehand.

However, the requirements for the file underneath the `<file_path` argument are as follows:
* The file needs to be encoded in UTF-8
* The words need to be alphabetically sorted
* The words need to be separated with a newline character as the file is read line-wise

The tool automatically removes duplicates and entries with a size different from 5 bytes and converts unicode characters to ASCII using [any_ascii]. German umlauts receive a special treatment.

## Database
If you choose to set up a database to serve as word base, please take a closer look at the [diesel.rs] documentation. You need a working diesel_cli installation to proceed.

Step 1: Create a running postgres instance. For example by executing this [docker] command

`docker run --name fhdb -e POSTGRES_USER=fhcli -e POSTGRES_PASSWORD=fhcli_pass -p 5432:5432 -e POSTGRES_DB=fhdb -d postgres`

Step 2: Set `DATABASE_URL` in `.env`

`DATABASE_URL=postgres://fhcli:fhcli_pass@localhost:5432/fhdb`

Step 3: Run the diesel initialization

`diesel setup`

Step 4: Import your word base into the database. You can use the existing text word base in the resources folder.

`cargo run --bin import res/en/word_base.txt en`

Et voilà! Enjoy additional features. A correctly guessed word will be marked as `used` in the database and won't show up a second time.

[diesel.rs]: http://diesel.rs/guides/getting-started 
[docker]: https://www.docker.com/
[any_ascii]: https://github.com/anyascii/anyascii