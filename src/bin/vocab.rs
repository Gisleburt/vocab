use std::path::{Path, PathBuf};

use structopt::StructOpt;

use vocab::{VocabStore, VocabStoreError};

/// Vocab app
#[derive(StructOpt)]
struct VocabApp {
    #[structopt(long)]
    pub init: bool,
}

const SQLITE_FILE: &str = "vocab.sqlite";

fn main() {
    let vocab_app = VocabApp::from_args();
    if vocab_app.init {
        initialise_and_exit();
    }

    let vocab = get_vocab_store();
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
    println!("Initialising database");
    let vocab_store = VocabStore::init(SQLITE_FILE)
        .expect("Could not init store");
    std::process::exit(0);
}
