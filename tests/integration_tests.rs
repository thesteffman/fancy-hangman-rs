use fhcli::util::io::*;
use crate::tools::{fill, get_sample_words, setup, teardown};

#[test]
fn test_read_random_word() {
    let file_path = setup();

    fill(file_path.as_str(), get_sample_words());

    let word_base = TextWordBase::new(file_path);

    match word_base.get_random_word() {
        Some(word) =>
            assert!(get_sample_words().contains(&word.word.as_str())),
        None => assert!(false)
    }

    teardown(word_base.wordbase_file_path);
}

#[test]
fn test_create_word() {

}

#[test]
fn test_find_word() {

}

mod tools {
    use std::env::temp_dir;
    use std::fs::{File, OpenOptions, remove_file};
    use std::io::Write;
    use uuid::Uuid;

    pub fn setup() -> String {
        let mut tmp_dir = temp_dir();
        let tmp_file_name = format!("{}.txt", Uuid::new_v4());
        tmp_dir.push(&tmp_file_name);

        File::create(tmp_dir);

        String::from(tmp_dir.as_os_str())
    }

    pub fn fill(file_path: &str, sample_words: Vec<&str>) {
        let mut tmp_dir = temp_dir();
        tmp_dir.push(file_path);

        let file_result =  OpenOptions::new()
            .append(true)
            .open(tmp_dir);

        match file_result {
            Ok(mut file) => {
                for word in sample_words {
                    file.write(word.as_ref()).unwrap();
                }
            }
            Err(e) => panic!("Error setting up integration test:\n{}", e)
        };
    }

    pub fn get_sample_words() -> Vec<&'static str> {
        vec!["rusty", "fishy", "busty", "lusty"]
    }

    pub fn teardown(file_path: String) {
        remove_file(file_path);
    }
}