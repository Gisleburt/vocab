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

use structopt::StructOpt;

use vocab::{Translation, VocabStore, VocabStoreError};

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

fn main() {
    if let Err(e) = app() {
        eprintln!("Something went wrong {:?}", e);
    }
}

fn app() -> Result<(), Box<dyn Error>> {
    match VocabApp::from_args().subcommand.unwrap_or(Command::Multi) {
        Command::Init => initialise_and_exit(),
        Command::Add { local, foreign } => {
            let translation = Translation::new(&local, &foreign);
            get_vocab_store().add(&translation)?;
        }
        Command::Single => {
            println!("Commands::Single");
            todo!()
        }
        Command::Multi => {
            println!("Commands::Multi");
            todo!()
        }
    };
    Ok(())
}

fn get_vocab_store() -> VocabStore {
    match VocabStore::from(SQLITE_FILE) {
        Ok(vs) => {
            return vs;
        }
        Err(VocabStoreError::NotInitialised) => {
            eprintln!("Not initialised, run: ");
            eprintln!("    vocab --init");
        }
        Err(e) => eprintln!("Unknown error {:?}", e),
    }
    std::process::exit(1);
}

fn initialise_and_exit() {
    match VocabStore::init(SQLITE_FILE) {
        Ok(_) => {
            println!("Database initialised");
            std::process::exit(0)
        }
        Err(_e @ VocabStoreError::AlreadyInitialised) => {
            eprintln!("Already initialised");
            std::process::exit(0)
        }
        Err(e) => eprintln!("Could not init store {}", e),
    }
    std::process::exit(1);
}
