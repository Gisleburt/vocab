//! # Vocab
//!
//! A command line application for learning vocabulary in new languages
//!
//! ## Usage:
//!
//! ### Initialising:
//!
//! `init` will create a bew vocab.sqlite file in the current directory. We recommend you use
//! directories to label which language you're learning.
//!
//! Example:
//!
//! ```shell
//! mkdir japanese
//! cd japanese
//! vocab init
//! ```
//!
//! This will help you separate and organise multiple languages.
//!
//! ### Adding new words
//!
//! `add <local> <foreign>` will let you add a new word as you learn it. `<local>` should be the
//! word in your own language, `<foreign>` should be the word in the language you are learning.
//!
//! Example:
//!
//! ```shell
//! vocab add japan 日本
//! ```
//!
//! ### Try a single word
//!
//! ToDo!
//!
//! ### Endless Mode
//!
//! ToDo!

use std::error::Error;
use std::fmt;
use std::io;

use structopt::StructOpt;

use vocab::{Guess, Translation, VocabStore, VocabStoreError};

/// Vocab app
#[derive(StructOpt)]
struct VocabApp {
    #[structopt(subcommand)]
    subcommand: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    /// Initialise the database
    Init,
    /// Add a new word to the database
    Add { local: String, foreign: String },
    /// Get a single word from the database
    Single,
    /// (default) Practice as many words as you like
    Multi,
}

const SQLITE_FILE: &str = "vocab.sqlite";

#[derive(Debug)]
enum AppError {
    VocabStoreError(VocabStoreError),
    NoTranslationsFound,
    IncorrectGuessInSingleMode,
    IoError(io::Error),
}

impl Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl From<VocabStoreError> for AppError {
    fn from(e: VocabStoreError) -> Self {
        AppError::VocabStoreError(e)
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::IoError(e)
    }
}

fn main() {
    // &* allows us to auto deref the Box<dyn Error>
    match app() {
        Ok(_) => std::process::exit(0),
        Err(AppError::VocabStoreError(VocabStoreError::AlreadyInitialised)) => {
            eprintln!("Already initialised");
        }
        Err(AppError::VocabStoreError(VocabStoreError::NotInitialised)) => {
            eprintln!("Not initialised, run: ");
            eprintln!("    vocab --init");
        }
        Err(AppError::VocabStoreError(VocabStoreError::DuplicateEntry)) => {
            eprintln!("Already stored that translation");
        }
        Err(AppError::NoTranslationsFound) => {
            eprintln!("No translations found, add with `vocab add <local> <foreign>");
        }
        Err(AppError::IncorrectGuessInSingleMode) => {
            // Nothing to do here, error message already given
        }
        Err(e) => eprintln!("Something went wrong {:?}", e),
    }
    std::process::exit(1);
}

fn app() -> Result<(), AppError> {
    match VocabApp::from_args().subcommand.unwrap_or(Command::Multi) {
        Command::Init => {
            VocabStore::init(SQLITE_FILE)?;
            println!("Database initialised");
        }
        Command::Add { local, foreign } => {
            let translation = Translation::new(&local, &foreign);
            VocabStore::from(SQLITE_FILE)?.add(&translation)?;
        }
        Command::Single => {
            let store = VocabStore::from(SQLITE_FILE)?;
            if let Some(guess_result) = store.guesses().next() {
                let mut guess = guess_result?;
                let result = handle_guess(&mut guess)?;
                store.save(&guess)?;
                return if result {
                    Ok(())
                } else {
                    Err(AppError::IncorrectGuessInSingleMode)
                };
            }
            return Err(AppError::NoTranslationsFound);
        }
        Command::Multi => {
            let store = VocabStore::from(SQLITE_FILE)?;
            for guess_result in store.guesses() {
                let mut guess = guess_result?;
                handle_guess(&mut guess)?;
                store.save(&guess)?;
                return Ok(());
            }
            return Err(AppError::NoTranslationsFound);
        }
    };
    Ok(())
}

fn handle_guess(guess: &mut Guess) -> Result<bool, AppError> {
    println!();
    println!("Translate: {}", guess.render());
    println!("Your guess: ");
    let user_guess = read_stdin()?;
    if guess.guess(&user_guess) {
        println!("Correct!");
        Ok(true)
    } else {
        println!(
            "Incorrect! The actual translation is {}",
            guess.render_translation()
        );
        Ok(false)
    }
}

fn read_stdin() -> Result<String, AppError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
