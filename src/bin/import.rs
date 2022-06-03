use std::env;
use std::env::temp_dir;
use std::fs::File;
use std::io::{Error, LineWriter, Write};
use uuid::Uuid;

use fhcli::util::env::{get_word_base, parse_app_language};
use fhcli::util::read::{get_reusable_buffered_reader};
use fhcli::util::lang::{AppLanguage, replace_unicode};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage:\n1st argument: Source file path\n2nd argument: Locale (default: en)");
    } else {
        let source_path = &args[1];
        let locale = &args[2];

        let app_language = parse_app_language(locale);

        match polish(source_path, app_language) {
            Ok(tmp_file_name) =>
                match import(tmp_file_name) {
                    Ok(counter) => println!("Added {} words to database. Have fun!", counter),
                    Err(error) => println!("An error occurred while importing. Import aborted\n{}", error)
                }
            Err(error) => println!("An error occurred while polishing. Import aborted\n{}", error)
        };
    }

    Ok(())
}

/// Read raw word list from source_path and polish with matching app_language strategy.
/// The word list needs to be an alphabetically sorted plain utf-8 encoded text file with newlines after each word.
/// Words of more than 5 bytes of size get discarded and duplicates get sorted out.
/// The polished list is then written to a temporary file located in the tmp directory of the filesystem.
///
/// See [temp_dir] documentation for more information.
///
/// # Arguments
///
/// * `src_path` - A string slice that holds the path of the file you want to import on the filesystem
/// * `app_language` - The language of the imported words. See [AppLanguage]
fn polish(source_path: &str, app_language: AppLanguage) -> Result<String, Error> {
    let mut tmp_dir = temp_dir();
    let tmp_file_name = format!("{}.txt", Uuid::new_v4());

    tmp_dir.push(&tmp_file_name);

    let out_file: Result<File, Error> = File::create(tmp_dir);

    match out_file {
        Ok(out_file) => {
            let mut reader = get_reusable_buffered_reader(source_path)?;
            let mut buffer = String::new();
            let mut writer: LineWriter<File> = LineWriter::new(out_file);

            println!("processing file {}", source_path);

            // compare word with previous valid selection to avoid duplicates
            let mut previous_word: String = String::with_capacity(5);

            while let Some(line) = reader.read_line(&mut buffer) {
                let word: String = line?.trim().to_lowercase();

                if word.len() == 5 && !previous_word.eq(&word) {
                    print!(".");

                    let polished: String = replace_unicode(&word, app_language) + "\n";

                    writer.write(polished.as_ref())?;

                    previous_word = word;
                }
            }

            println!("finished polishing");

            Ok(tmp_file_name)
        }
        Err(error) => Err(error)
    }
}

/// Import temporary file created by [polish] into the word base.
///
/// # Arguments
///
/// * `tmp_file_name` - A String that holds the name of the temp file created
fn import(tmp_file_name: String) -> Result<i32, Error> {
    let mut tmp_dir = temp_dir();
    tmp_dir.push(tmp_file_name);

    let word_base = get_word_base().unwrap();
    let mut reader = get_reusable_buffered_reader(tmp_dir)?;
    let mut buffer = String::new();

    println!("Importing...");

    let mut counter = 0;
    while let Some(line) = reader.read_line(&mut buffer) {
        let w_str: &str = line.unwrap().trim();

        word_base.create_word(w_str);

        // println!("Added word {}", w.word);
        print!(".");

        counter += 1;
    }

    Ok(counter)
}