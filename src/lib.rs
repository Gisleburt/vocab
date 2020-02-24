#[macro_use]
extern crate diesel;

pub use translation::Translation;
pub use vocab_store::{VocabStore, VocabStoreError};

mod schema;
mod translation;
mod vocab_store;
