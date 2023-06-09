//! # simple-localization
//! ## Build
//! You should create a `LOCALIZATION_DIR` environment variable to set the path where this crate will look for localization files.
//! Example build command:
//! ```bash
//! LOCALIZATION_DIR=/home/user/Projects/rust_project/localization/ cargo build
//! ```
//! Example file tree:
//! ```bash
//! ├── Cargo.toml
//! ├── localization
//! │   ├── ar_QA # translation files here
//! │   ├── en_US
//! │   └── tr_TR
//! └── src
//!     └── lib.rs
//! ```
//! Example translation file content `tr_TR`:
//! ```
//! "Hello" => "Merhaba"
//! "How are you?" => "Nasılsın?"
//! "This is a long text" => "Bu uzun bir yazı"
//! 
//! #"This is a multiline text.
//! 
//! You can write anything you want here.
//! 
//! You don't need to use \n to show newlines.
//! 
//! The translation of this is next the quoted text."#
//! =>
//! #"Bu bir çok satırlı yazı.
//! 
//! Buraya istediğin her şeyi yazabilirsin.
//! 
//! Yeni satırları göstermek için \n kullanman gerekmez.
//! 
//! Bu yazının çevirisi bir sonraki tırnak içindeki yazıdır."#
//! ```
//! ## Usage
//! - Use `tr("Text")` if you want to use user's computer's LANG environment variable(like: `LANG=en_US.UTF-8`) at startup to determine their system language and translate the program to that language.
//! - Use `trl("Text", "en_US")` if you want to use your own variable to store user's language. For example you can change the program's language by changing the second parameter of `trl` without restarting the app. 

use std::{collections::HashMap, env};

use include_dir::{include_dir, Dir};
use lazy_regex::{lazy_regex, Lazy, Regex};
use lazy_static::lazy_static;

lazy_static! {
    static ref TRANSLATIONS_HASHMAP: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut all_translations: HashMap<&'static str, HashMap<&'static str, &'static str>> = HashMap::new();

        // Read all files and add them to the hashmap:
        static LOCALIZATION_DIR__: Dir<'_> = include_dir!("$LOCALIZATION_DIR");
        for file in LOCALIZATION_DIR__.files() {
            let translation: HashMap<&'static str, &'static str> = create_translation_hashmap(file.contents_utf8().unwrap());

            all_translations.insert(file.path().file_name().unwrap().to_str().unwrap(), translation);
        }

        all_translations
    };
}

static ONE_LINE_REGEX: Lazy<Regex> = lazy_regex!(r#""([\w\W]+?)"\s*=>\s*"([\w\W]+?)""#);
static MULTI_LINE_REGEX: Lazy<Regex> = lazy_regex!(r#"\#"([\w\W]+?)"\#\s*=>\s*\#"([\w\W]+?)"\#"#);

fn create_translation_hashmap( localization_string: &str ) -> HashMap<&str, &str> {
    let mut translations: HashMap<&str, &str> = HashMap::new();

    // One line
    for cap in ONE_LINE_REGEX.captures_iter(localization_string) {
        if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
            translations.insert(key.as_str(), value.as_str());
        }
    }

    // Multi Line
    for cap in MULTI_LINE_REGEX.captures_iter(localization_string) {
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
