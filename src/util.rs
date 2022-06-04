/// Provides file i/o with a reusable buffered reader
pub mod read {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    /// Wraps a [BufReader] with additional functionality
    pub struct BufferedReader {
        reader: BufReader<File>,
    }

    impl BufferedReader {
        fn open(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            Ok(Self { reader })
        }

        /// Returns the next line in the reader. The method can be conveniently used in a loop.
        ///
        /// # Arguments
        /// * `buffer` - A mutable String that can be reused
        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<std::io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }

    /// Returns a [BufferedReader] for the file underneath the specified path.
    pub fn get_reusable_buffered_reader(path: impl AsRef<std::path::Path>) -> std::io::Result<BufferedReader> {
        self::BufferedReader::open(path)
    }
}

/// Handles localization issues and unicode to ASCII conversion
pub mod lang {
    use any_ascii::any_ascii;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum AppLanguage {
        DE,
        EN
    }

    /// Eliminate non-ASCII characters.
    /// Replace common german special characters with their matching counterparts.
    /// The parameter is expected to be lower case.
    pub fn replace_unicode(word: &str, app_language: AppLanguage) -> String {
        match app_language {
            AppLanguage::DE => {
                // replace umlauts for german language
                let without_umlauts = replace_umlauts(word);

                any_ascii(without_umlauts.as_str())
            }
            _ => any_ascii(word)
        }
    }

    /// Replace german umlaut characters with their logical counterparts. The parameter is expected to be lowercase.
    ///
    /// * 'ä' -> "ae"
    /// * 'ö' -> "oe"
    /// * 'ü' -> "ue"
    fn replace_umlauts(word: &str) -> String {
        word.replace("ä", "ae").replace("ö", "oe").replace("ü", "ue")
    }

    #[cfg(test)]
    mod tests {
        use super::AppLanguage;
        use super::replace_unicode;

        /// Test correct replacement of german special characters
        #[test]
        fn test_replace_unicode() {
            assert_eq!(replace_unicode("schön", AppLanguage::DE), "schoen");
            assert_eq!(replace_unicode("geschoß", AppLanguage::DE), "geschoss");
            assert_eq!(replace_unicode("zäh", AppLanguage::DE), "zaeh");
            assert_eq!(replace_unicode("lüge", AppLanguage::DE), "luege");

            assert_eq!(replace_unicode("schön", AppLanguage::EN), "schon");
            assert_eq!(replace_unicode("geschoß", AppLanguage::EN), "geschoss");
            assert_eq!(replace_unicode("zäh", AppLanguage::EN), "zah");
            assert_eq!(replace_unicode("lüge", AppLanguage::EN), "luge");
        }
    }
}

/// Access environment settings
pub mod env {
    use std::env;
    use std::io::Error;

    use dotenv::dotenv;

    use super::db::DbWordBase;
    use super::lang::AppLanguage;
    use super::io::WordBase;
    use super::io::TextWordBase;

    /// Returns a [WordBase] based on the env flags
    ///
    /// * if DATABASE_URL is present in .env, a [DbWordBase] will be created and returned
    /// * if no DATABASE_URL is present .env, a WORDBASE_FILE must be set; then a [TextWordBase] will be created and returned
    pub fn get_word_base() -> Result<Box<dyn WordBase>, Error> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL");

        match database_url {
            Ok(url) => Ok(Box::new(DbWordBase::new(url))),
            Err(_e) => {
                let wordbase_file = env::var("WORDBASE_FILE").expect("WORDBASE_FILE must be set");

                Ok(Box::new(TextWordBase::new(wordbase_file)))
            }
        }
    }

    /// Returns MAX_GUESSES set in .env
    pub fn get_max_guesses() -> i32 {
        dotenv().ok();

        let max_guesses_str: String = env::var("MAX_GUESSES").expect("MAX_GUESSES must be set");

        max_guesses_str.parse().unwrap()
    }

    /// Returns parsed LOCALE set in .env
    pub fn get_app_language() -> AppLanguage {
        dotenv().ok();

        let locale = env::var("LOCALE").expect("LOCALE must be set");
        parse_app_language(locale.as_str())
    }

    /// Parse locale flag to AppLanguage representation. Defaults to [AppLanguage::EN]
    pub fn parse_app_language(locale: &str) -> AppLanguage {
        match locale {
            "de" => AppLanguage::DE,
            "en" => AppLanguage::EN,
            _ => AppLanguage::EN
        }
    }

    #[cfg(test)]
    mod tests {
        use super::AppLanguage;
        use super::parse_app_language;

        /// Test app language flag conversion
        #[test]
        fn test_parse_app_language() {
            assert_eq!(parse_app_language("de"), AppLanguage::DE);
            assert_eq!(parse_app_language("en"), AppLanguage::EN);
            assert_eq!(parse_app_language(""), AppLanguage::EN);
        }
    }
}

/// Provides access to the wordbase
pub mod io {
    use std::fs::{File, OpenOptions};
    use std::io::{BufRead, BufReader, Error, Write};

    use rand::prelude::IteratorRandom;

    use crate::models::Word;

    use super::read::BufferedReader;
    use super::read::get_reusable_buffered_reader;

    /// Provides basic functions for reading and writing from and to a word base
    pub trait WordBase {
        fn get_random_word(&self) -> Option<Word>;
        fn find_word(&self, text: &str) -> Option<Word>;
        fn create_word(&self, text: &str);
        fn update_word(&self, word: Word);
    }

    /// Provides a word base represented by a text file
    pub struct TextWordBase {
        pub wordbase_file_path: String
    }

    impl TextWordBase {
        /// Creates word base based on the file given
        ///
        /// # Arguments
        /// * `file_path` - A String representing the path to the word base file on the filesystem
        pub fn new(file_path: String) -> TextWordBase {
            TextWordBase { wordbase_file_path: file_path }
        }
    }

    impl WordBase for TextWordBase {
        /// Get [Word] from a random line of the wordbase using reservoir sampling
        fn get_random_word(&self) -> Option<Word> {
            let file_result = File::open(&self.wordbase_file_path);

            match file_result {
                Ok(file) => {
                    let random_line = BufReader::new(file).lines().choose(&mut rand::thread_rng());

                    match random_line {
                        Some(line) => Some(Word { id: 0, word: line.unwrap(), used: false }),
                        None => None
                    }
                }
                Err(e) => {
                    println!("Error reading from the wordbase:\n{}", e);
                    None
                }
            }
        }

        /// Search the wordbase for a specific [Word]
        fn find_word(&self, text: &str) -> Option<Word> {
            let reader_result: Result<BufferedReader, Error> = get_reusable_buffered_reader(&self.wordbase_file_path);

            match reader_result {
                Ok(mut reader ) => {
                    let mut buffer = String::new();
                    let mut result = None;

                    while let Some(line) = reader.read_line(&mut buffer) {
                        let w_str: &str = line.unwrap().trim();

                        if text.eq(w_str) {
                            result = Some(Word { id: 0, word: String::from(w_str), used: false });
                            break;
                        }
                    }

                    result
                },
                Err(e) => {
                    println!("Error when looking for '{}' in the wordbase:\n{}", text, e);
                    None
                }
            }
        }

        /// Append a [Word] to the wordbase
        fn create_word(&self, text: &str) {
            match self.find_word(text) {
                Some(_) => {},
                None => {
                    let file_result =  OpenOptions::new()
                        .append(true)
                        .open(&self.wordbase_file_path);

                    match file_result {
                        Ok(mut file) => {
                            file.write(format!("{}\n", text).as_ref()).unwrap();
                        }
                        Err(e) => println!("Error when writing '{}' to the wordbase:\n{}", text, e)
                    };
                }
            }
        }

        /// Not implemented for text wordbase
        fn update_word(&self, _word: Word) {}
    }
}

/// Encapsulates a database based word base
pub mod db {
    use diesel::{Connection, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
    use diesel::dsl::sql;

    use crate::models::{NewWord, Word};

    use super::io::WordBase;

    /// Provides a word base represented by a postgres database
    pub struct DbWordBase {
        conn: PgConnection,
    }

    impl DbWordBase {
        /// Creates word base based on the database url given
        ///
        /// # Arguments
        /// * `db` - A String representing the url to the database
        pub fn new(db_url: String) -> DbWordBase {
            DbWordBase { conn: PgConnection::establish(&db_url)
                .expect(&format!("Error connecting to database {}", db_url)) }
        }
    }

    impl WordBase for DbWordBase {
        /// Get a random unused [Word] from the database
        fn get_random_word(&self) -> Option<Word> {
            use crate::schema::words::dsl::*;

            let db_result = words.filter(used.eq(false))
                .order(sql::<()>("RANDOM()"))
                .limit(1)
                .get_result(&self.conn)
                .optional();

            match db_result {
                Ok(db_word) => db_word,
                Err(error) => {
                    println!("Error when reading from the database\n{}", error);

                    None
                }
            }
        }

        /// Search the database for a specific [Word] regardless of the 'used' flag
        fn find_word(&self, text: &str) -> Option<Word> {
            use crate::schema::words::dsl::*;

            let db_result = words.filter(word.eq(text))
                .get_result::<Word>(&self.conn)
                .optional();

            match db_result {
                Ok(db_word) => db_word,
                Err(error) => {
                    println!("Error when looking for '{}' in the database:\n{}", text, error);

                    None
                }
            }
        }

        /// Insert a new [Word] into the database
        fn create_word(&self, text: &str) {
            use crate::schema::words;

            let new_word = NewWord {
                word: text
            };

            let db_result = diesel::insert_into(words::table)
                .values(new_word)
                .execute(&self.conn);

            match db_result {
                Ok(_) => println!("Added '{}' to the database!", text),
                Err(e) => println!("Error when writing '{}' to the database:\n{}", text, e)
            };
        }

        /// Updates the 'used' flag of the given [Word]
        fn update_word(&self, word: Word) {
            use crate::schema::words;
            use crate::schema::words::id;
            use crate::schema::words::used;

            let db_result = diesel::update(words::table)
                .filter(id.eq(word.id))
                .set(used.eq(true))
                .execute(&self.conn);

            match db_result {
                Ok(result_size) => {
                    if result_size <= 0 {
                        println!("Error when updating '{}' in the database: No rows were affected.", word.word)
                    } else {
                        println!("Updated '{}' in the database!", word.word)
                    }
                }
                Err(e) => println!("Error when updating '{}' in the database:\n{}", word.word, e)
            }
        }
    }
}