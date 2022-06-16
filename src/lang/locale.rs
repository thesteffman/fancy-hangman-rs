use any_ascii::any_ascii;

/// Eliminate non-ASCII characters.
/// Replace common german special characters with their matching counterparts.
/// The parameter is expected to be lower case.
pub fn replace_unicode(word: &str, lang: &str) -> String {
    match lang {
        "de" => {
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
#[test]
fn test_replace_unicode() {
    assert_eq!(replace_unicode("schön", "de"), "schoen");
    assert_eq!(replace_unicode("geschoß", "de"), "geschoss");
    assert_eq!(replace_unicode("zäh", "de"), "zaeh");
    assert_eq!(replace_unicode("lüge", "de"), "luege");

    assert_eq!(replace_unicode("schön", "en"), "schon");
    assert_eq!(replace_unicode("geschoß", "en"), "geschoss");
    assert_eq!(replace_unicode("zäh", "en"), "zah");
    assert_eq!(replace_unicode("lüge", "en"), "luge");
}

#[cfg(test)]
#[test]
fn test_replace_umlauts() {
    assert_eq!(replace_umlauts("schön"), "schoen");
    assert_eq!(replace_umlauts("zäh"), "zaeh");
    assert_eq!(replace_umlauts("lüge"), "luege");

    assert_ne!(replace_umlauts("schön"), "schon");
    assert_ne!(replace_umlauts("zäh"), "zah");
    assert_ne!(replace_umlauts("lüge"), "luge");
}