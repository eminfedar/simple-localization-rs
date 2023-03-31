# simple-localization
Simple localization library

---
## Build
You should create a `LOCALIZATION_DIR` environment variable to set the path where this crate will look for localization files.
Example build command:
```bash
LOCALIZATION_DIR=/home/user/Projects/rust_project/localization/ cargo build
```
Example file tree:
```bash
├── Cargo.toml
├── localization
│   ├── ar_QA # translation files here
│   ├── en_US
│   └── tr_TR
└── src
    └── lib.rs
```
Example translation file content `tr_TR`:
```
"Hello" => "Merhaba"
"How are you?" => "Nasılsın?"
"This is a long text" => "Bu uzun bir yazı"

#"This is a multiline text.

You can write anything you want here.

You don't need to use \n to show newlines.

The translation of this is next the quoted text."#
=>
#"Bu bir çok satırlı yazı.

Buraya istediğin her şeyi yazabilirsin.

Yeni satırları göstermek için \n kullanman gerekmez.

Bu yazının çevirisi bir sonraki tırnak içindeki yazıdır."#
```
## Usage
- Use `tr("Text")` if you want to use user's computer's LANG environment variable(like: `LANG=en_US.UTF-8`) at startup to determine their system language and translate the program to that language.
- Use `trl("Text", "en_US")` if you want to use your own variable to store user's language. For example you can change the program's language by changing the second parameter of `trl` without restarting the app. 

`tr()` usage in your `main.rs`:
```rust
use simple_localization::tr;

// localization/tr_TR file exists and `LANG=tr_TR.UTF-8`
let text:&str = tr("Hello"); // "Merhaba"

// localization/tr_TR file doesn't exists and `LANG=tr_TR.UTF-8`
let text:&str = tr("Hello"); // "Hello"

// localization/ar_QA file doesn't exists and `LANG=ar_QA.UTF-8`
let text:&str = tr("Hello"); // "Hello"
```

`trl()` usage in your `main.rs`:
```rust
use simple_localization::trl;

// localization/tr_TR file exists
let text:&str = trl("Hello", "tr_TR"); // "Merhaba"

// localization/ar_QA file exists
let text:&str = trl("Hello", "ar_QA"); // "مرحبًا"

// localization/tr_TR file doesn't exists
let text:&str = trl("Hello", "tr_TR"); // "Hello"
```
