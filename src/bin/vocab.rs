use std::error::Error;

use structopt::StructOpt;

use vocab::{Translation, VocabStore, VocabStoreError};

/// Vocab app
#[derive(StructOpt)]
struct VocabApp {
    #[structopt(subcommand)]
    subcommand: Option<Command>
}

#[derive(StructOpt)]
enum Command {
    Init,
    Add {
        local: String,
        foreign: String,
    },
    Single,
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
            get_vocab_store().store(translation)?;
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
        },
        Err(VocabStoreError::NotInitialised) => {
            eprintln!("Not initialised, run: ");
            eprintln!("    vocab --init");
        },
        Err(e) => eprintln!("Unknown error {:?}", e),
    }
    std::process::exit(1);
}

fn initialise_and_exit() {
    match VocabStore::init(SQLITE_FILE) {
        Ok(_) => {
            println!("Database initialised");
            std::process::exit(0)
        },
        Err(e @ VocabStoreError::AlreadyInitialised) => {
            eprintln!("Already initialised");
            std::process::exit(0)
        },
        Err(e) => eprintln!("Could not init store {}", e),
    }
    std::process::exit(1);
}
