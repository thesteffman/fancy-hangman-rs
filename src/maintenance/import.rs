use std::env::temp_dir;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, LineWriter, Write};
use std::time::Instant;

use console::{Emoji, style};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use uuid::Uuid;

use crate::dictionary::{Dictionary, DictionaryEntry, get_dictionary};
use crate::db::db_dictionary::DbDictionary;
use crate::lang::locale::{AppLanguage, replace_unicode};

static BOOKMARK: Emoji<'_, '_> = Emoji("🔖  ", "");
static MINIDISC: Emoji<'_, '_> = Emoji("💽  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

pub fn do_import(source_file: String, app_language: AppLanguage) -> std::io::Result<()> {
    let dictionary = get_dictionary(app_language);

    let started = Instant::now();

    println!(
        "{} {}Polishing file...",
        style("[1/2]").bold().dim(),
        BOOKMARK
    );

    println!(
        "{} {}Importing file...",
        style("[2/2]").bold().dim(),
        MINIDISC
    );

    let progress_polish = setup_spinner();
    progress_polish.set_message(format!("Processing {}...", source_file));

    let meta_data = polish(&source_file, app_language)?;

    progress_polish.finish_with_message(format!("Finished processing {}. Importing...", source_file));

    let progress_import = ProgressBar::new(meta_data.1);

    let counter = import(meta_data.0, dictionary, &progress_import)?;

    progress_polish.finish_and_clear();
    progress_import.finish_and_clear();

    println!("{} Done in {}. Added {} words to the dictionary!", SPARKLE, HumanDuration(started.elapsed()), counter);

    Ok(())
}

/// Read raw word list from source_path and polish with matching app_language strategy.
/// The polished list is then written to a temporary file located in the tmp directory of the filesystem.
///
/// See [temp_dir] documentation for more information.
///
/// # Arguments
///
/// * `src_path` - A string slice that holds the path of the file you want to maintenance on the filesystem
/// * `app_language` - The language of the imported words. See [AppLanguage]
fn polish(source_path: &str, app_language: AppLanguage) -> Result<(String, u64), Error> {
    let tmp_file_name = format!("{}/{}.txt", temp_dir().to_str().unwrap(), Uuid::new_v4());
    let out_file: Result<File, Error> = File::create(&tmp_file_name);

    match out_file {
        Ok(out_file) => {
            let buf_reader = BufReader::new(File::open(source_path).unwrap());
            let mut writer: LineWriter<File> = LineWriter::new(out_file);

            let mut counter = 0;

            for line_result in buf_reader.lines() {
                let polished = replace_unicode(line_result.unwrap().to_lowercase().as_str(), app_language);

                if polished.len() == 5 {
                    writer.write(polished.as_ref())?;
                    writer.write(b"\n")?;

                    counter += 1;
                }
            }

            Ok((tmp_file_name, counter))
        }
        Err(error) => Err(error)
    }
}

/// Import temporary file created by [polish] into the dictionary.
/// Avoid duplicates when inserting a [WordEntry] into the dictionary.
///
/// # Arguments
///
/// * `tmp_file_name` - A String that holds the name of the temp file created
fn import(tmp_file_name: String, dictionary: DbDictionary, progress_bar: &ProgressBar) -> Result<i32, Error> {
    let buf_reader = BufReader::new(File::open(tmp_file_name).unwrap());

    let mut counter = 0;
    for line_result in buf_reader.lines() {
        let line = line_result.unwrap();

        match dictionary.create_word(DictionaryEntry {
            word: line.to_lowercase(),
            guessed: false
        }) {
            None => {},
            Some(_) => {
                counter += 1;
            }
        }

        progress_bar.inc(1);
    }

    Ok(counter)
}

fn setup_spinner() -> ProgressBar {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(120);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{prefix:.bold.dim} {spinner:.green} {msg}"));

    progress_bar
}