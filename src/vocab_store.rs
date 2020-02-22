use std::error::Error;
use std::io;
use std::path::Path;

use diesel::{Connection, ConnectionError, RunQueryDsl, SqliteConnection};

use crate::translation::Translation;

const INIT: &str = include_str!("migrations/2020-02-22_vocab_table.sql");

#[derive(Debug)]
pub enum VocabStoreError {
    ConnectionError(ConnectionError),
    NotInitialised,
    UnexpectedError(Box<dyn Error>),
}

impl From<ConnectionError> for VocabStoreError {
    fn from(e: ConnectionError) -> Self {
        VocabStoreError::ConnectionError(e)
    }
}

impl From<std::io::Error> for VocabStoreError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            io::ErrorKind::NotFound => VocabStoreError::NotInitialised,
            _ => VocabStoreError::UnexpectedError(Box::new(e))
        }
    }
}

type VSResult<T> = Result<T, VocabStoreError>;

pub struct VocabStore(SqliteConnection);

impl VocabStore {
    pub fn from(file: &str) -> VSResult<VocabStore> {
        if !Path::new(file).exists() {
            return Err(VocabStoreError::NotInitialised)
        }
        let connection = SqliteConnection::establish(file)?;
        Ok(VocabStore(connection))
    }

    pub fn init(file: &str) -> VSResult<VocabStore> {
        let connection = SqliteConnection::establish(file)?;
        diesel::sql_query(INIT).execute(&connection);
        Ok(VocabStore(connection))
    }

    pub fn store(&self, translation: Translation) -> VSResult<()> {
        todo!()
    }

    pub fn find(&self, word: String) -> VSResult<Translation> {
        todo!()
    }

    pub fn get_all(&self) -> VSResult<Vec<Translation>> {
        todo!()
    }
}
