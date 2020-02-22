use diesel::{Connection, ConnectionError, RunQueryDsl, SqliteConnection};

use crate::translation::Translation;

const INIT: &str = include_str!("migrations/2020-02-22_vocab_table.sql");

#[derive(Debug)]
pub enum VocabStoreError {
    ConnectionError(ConnectionError),
}

impl From<ConnectionError> for VocabStoreError {
    fn from(e: ConnectionError) -> Self {
        VocabStoreError::ConnectionError(e)
    }
}

type VSResult<T> = Result<T, VocabStoreError>;

pub struct VocabStore(SqliteConnection);

impl VocabStore {
    pub fn new(file: &str) -> VSResult<VocabStore> {
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
