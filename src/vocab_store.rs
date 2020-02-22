use core::fmt;
use std::error::Error;
use std::io;
use std::path::Path;

use diesel::{Connection, ConnectionError, result::Error as DieselError, RunQueryDsl, SqliteConnection};

use crate::translation::Translation;

const INIT: &str = include_str!("migrations/2020-02-22_vocab_table.sql");

#[derive(Debug)]
pub enum VocabStoreError {
    ConnectionError(ConnectionError),
    NotInitialised,
    AlreadyInitialised,
    UnexpectedError(Box<dyn Error>),
    DatabaseError(DieselError),
}

impl fmt::Display for VocabStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for VocabStoreError {}

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

impl From<DieselError> for VocabStoreError {
    fn from(e: DieselError) -> Self {
        VocabStoreError::DatabaseError(e)
    }
}

type VSResult<T> = Result<T, VocabStoreError>;

pub struct VocabStore(SqliteConnection);

impl VocabStore {
    pub fn from(file: &str) -> VSResult<VocabStore> {
        if !Path::new(file).exists() {
            return Err(VocabStoreError::NotInitialised);
        }
        let connection = SqliteConnection::establish(file)?;
        Ok(VocabStore(connection))
    }

    pub fn init(file: &str) -> VSResult<VocabStore> {
        if Path::new(file).exists() {
            return Err(VocabStoreError::AlreadyInitialised);
        }
        let connection = SqliteConnection::establish(file)?;
        diesel::sql_query(INIT).execute(&connection);
        Ok(VocabStore(connection))
    }

    pub fn store(&self, translation: Translation) -> VSResult<()> {
        diesel::insert_into(crate::schema::translations::table)
            .values(&translation)
            .execute(&self.0)?;
        Ok(())
    }

    pub fn find(&self, word: String) -> VSResult<Translation> {
        todo!()
    }

    pub fn get_all(&self) -> VSResult<Vec<Translation>> {
        todo!()
    }
}
