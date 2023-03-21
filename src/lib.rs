use std::{collections::HashMap, env};

use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TRANSLATIONS_HASHMAP: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut all_translations: HashMap<&'static str, HashMap<&'static str, &'static str>> = HashMap::new();

        // Read all files and add them to the hashmap:
        static LOCALIZATION_DIR__: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/localization");
        for file in LOCALIZATION_DIR__.files() {
            let translation: HashMap<&'static str, &'static str> = create_translation_hashmap(file.contents_utf8().unwrap());

            all_translations.insert(file.path().file_name().unwrap().to_str().unwrap(), translation);
        }

        all_translations
    };
}


fn create_translation_hashmap( localization_string: &str ) -> HashMap<&str, &str> {
    let mut translations: HashMap<&str, &str> = HashMap::new();

    let one_line = Regex::new(r#""([\w\W]+?)"\s*=>\s*"([\w\W]+?)""#).unwrap();
    let multi_line = Regex::new(r#"\#"([\w\W]+?)"\#\s*=>\s*\#"([\w\W]+?)"\#"#).unwrap();

    // One line
    for cap in one_line.captures_iter(localization_string) {
        if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
            translations.insert(key.as_str(), value.as_str());
        }
    }

    // Multi Line
    for cap in multi_line.captures_iter(localization_string) {
        if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
            translations.insert(key.as_str(), value.as_str());
        }
    }

    translations
}

/// Get translation of the `text` in a spesific language
/// If translation exists returns the translation
/// else returns the `text` back.
/// ```rust,ignore
/// use simple_localization::trl;
/// 
/// // localization/tr_TR file exists
/// let text:&str = trl("Hello", "tr_TR"); // "Merhaba"
/// 
/// // localization/ar_QA file exists
/// let text:&str = trl("Hello", "ar_QA"); // "مرحبًا"
/// 
/// // localization/tr_TR file doesn't exists
/// let text:&str = trl("Hello", "tr_TR"); // "Hello"
/// ```
/// 
pub fn trl<'a, 'b>(text: &'a str, lang: &'b str) -> &'a str {
    match TRANSLATIONS_HASHMAP.get(lang) {
        Some(language_translations) => {
            match language_translations.get(text) {
                Some(&value) => value,
                None => {
                    eprintln!("Translation Error: No translation of '{text}' exists in '{lang}' language");
                    return text;
                }
            }
        }
        None => {
            eprintln!("Translation Error: localization/{lang} doesn't exist");
            return text;
        }
    }
}

/// Get translation of the `text` in system's language(`env::var("LANG")`)
/// If translation exists returns the translation
/// else returns the `text` back.
/// ```rust,ignore
/// use simple_localization::tr;
/// 
/// // localization/tr_TR file exists and `LANG=tr_TR.UTF-8`
/// let text:&str = tr("Hello"); // "Merhaba"
/// 
/// // localization/tr_TR file doesn't exists and `LANG=tr_TR.UTF-8`
/// let text:&str = tr("Hello"); // "Hello"
/// 
/// // localization/tr_TR file exists and `LANG=ar_QA.UTF-8`
/// let text:&str = tr("Hello"); // "Hello"
/// ```
/// 
pub fn tr(text: &str) -> &str {
    let lang = match env::var("LANG") {
        Ok(l) => l, // this returns "en_US.UTF-8"
        Err(_) => {
            eprintln!("Translation Error: 'LANG' environment variable doesn't exist");
            return text;
        }
    };

    let lang_vec:Vec<&str> = lang.split(".").collect();
    let lang_str:&str = match lang_vec.first() {
        Some(&l) => l, // this returns "en_US"
        None => {
            eprintln!("Translation Error: 'LANG' environment variable is not suitable to parse: {lang} (example: en_US.UTF-8)");
            return text;
        }
    };

    println!("LANG is: {lang_str}");

    trl(text, lang_str)
}


#[cfg(test)]
mod tr_tests {
    use super::*;

    #[test]
    fn single_word() {
        assert_eq!(tr("Hello"), "Merhaba");
    }

    

    #[test]
    fn long_sentence() {
        assert_eq!(tr("How are you?"), "Nasılsın?");
        assert_eq!(tr("This is a long text"), "Bu uzun bir yazı");
    }

    #[test]
    fn multi_line() {
        assert_eq!(
            tr(r#"This is a multiline text.

You can write anything you want here.

Don't need to use \n.

The translation of this is next the quoted text."#),
r#"Bu bir çok satırlı yazı.

Buraya istediğin her şeyi yazabilirsin.

\n kullanman gerekmez.

Bu yazının çevirisi bir sonraki tırnak içindeki yazıdır."#
        );
    }    
}

#[cfg(test)]
mod trl_tests {
    use super::*;

    #[test]
    fn single_word() {
        assert_eq!(trl("Hello", "tr_TR"), "Merhaba");
    }

    

    #[test]
    fn long_sentence() {
        assert_eq!(trl("How are you?", "tr_TR"), "Nasılsın?");
        assert_eq!(trl("This is a long text", "tr_TR"), "Bu uzun bir yazı");
    }

    #[test]
    fn multi_line() {
        assert_eq!(
            trl(r#"This is a multiline text.

You can write anything you want here.

Don't need to use \n.

The translation of this is next the quoted text."#, "tr_TR"),
r#"Bu bir çok satırlı yazı.

Buraya istediğin her şeyi yazabilirsin.

\n kullanman gerekmez.

Bu yazının çevirisi bir sonraki tırnak içindeki yazıdır."#
        );
    }    
}
