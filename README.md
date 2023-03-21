# simple-localization-rs
Simple localization library

---
## Usage
This directory tree must be in your project:
```bash
├── Cargo.lock
├── Cargo.toml
├── localization
│   └── tr_TR # Add files here
└── src
    └── lib.rs
```
Inside the localization files *(example: `localization/tr_TR`):*
```c
// One Line Example
"Hello" => "Merhaba"
"How are you?" => "Nasılsın?"
"This is a long text" => "Bu uzun bir yazı"

// Multiline Example
#"This is a multiline text.

You can write anything you want here.

Don't need to use \n.

The translation of this is next the quoted text."#
=>
#"Bu bir çok satırlı yazı.

Buraya istediğin her şeyi yazabilirsin.

\n kullanman gerekmez.

Bu yazının çevirisi bir sonraki tırnak içindeki yazıdır."#
```
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
