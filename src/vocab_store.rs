use core::fmt;
use std::error::Error;
use std::io;
use std::path::Path;

use diesel::result::DatabaseErrorKind;
use diesel::{
    result::Error as DieselError, Connection, ConnectionError, RunQueryDsl, SqliteConnection,
};

use crate::guesses::Guesses;
use crate::translation::Translation;

const INIT: &str = include_str!("migrations/2020-02-22_vocab_table.sql");

#[derive(Debug)]
pub enum VocabStoreError {
    ConnectionError(ConnectionError),
    NotInitialised,
    AlreadyInitialised,
    DuplicateEntry,
    DatabaseError(DieselError),
    UnexpectedError(Box<dyn Error>),
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
            _ => VocabStoreError::UnexpectedError(Box::new(e)),
        }
    }
}

impl From<DieselError> for VocabStoreError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                VocabStoreError::DuplicateEntry
            }
            _ => VocabStoreError::DatabaseError(e),
        }
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
        diesel::sql_query(INIT).execute(&connection)?;
        Ok(VocabStore(connection))
    }

    pub fn add(&self, translation: &Translation) -> VSResult<()> {
        diesel::insert_into(crate::schema::translations::table)
            .values(translation)
            .execute(&self.0)?;
        Ok(())
    }

    pub fn save(&self, translation: &Translation) -> VSResult<()> {
        diesel::replace_into(crate::schema::translations::table)
            .values(translation)
            .execute(&self.0)?;
        Ok(())
    }

    pub fn guesses(&self) -> Guesses {
        Guesses::new(&self.0)
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use diesel::{Connection, RunQueryDsl, SqliteConnection};

    use crate::{Translation, VocabStore, VocabStoreError};

    const TEST_FILE: &str = "test.sqlite";

    #[test]
    fn test_from() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        SqliteConnection::establish(&TEST_FILE).unwrap();
        assert!(VocabStore::from(&TEST_FILE).is_ok());
    }

    #[test]
    fn test_from_error() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        match VocabStore::from(&TEST_FILE) {
            Err(VocabStoreError::NotInitialised) => {}
            _ => assert!(false, "VocabStore did not return NotInitialised error"),
        }
    }

    #[test]
    fn test_init() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        assert!(VocabStore::init(&TEST_FILE).is_ok());
    }

    #[test]
    fn test_init_error() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        SqliteConnection::establish(&TEST_FILE).unwrap();
        match VocabStore::init(&TEST_FILE) {
            Err(VocabStoreError::AlreadyInitialised) => {}
            _ => assert!(false, "VocabStore did not return AlreadyInitialised error"),
        }
    }

    #[test]
    fn test_add() {
        use crate::schema::translations::dsl::*;

        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        let vocab_store = VocabStore::init(&TEST_FILE).unwrap();
        let translation = Translation::new("yes", "はい");
        vocab_store.add(&translation).unwrap();

        let conn = SqliteConnection::establish(&TEST_FILE).unwrap();
        let t: Translation = translations.load(&conn).unwrap().pop().unwrap();

        assert_eq!(t.local, "yes");
        assert_eq!(t.foreign, "はい");
    }

    #[test]
    fn test_add_duplicate_local() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        let vocab_store = VocabStore::init(&TEST_FILE).unwrap();
        let translation = Translation::new("yes", "はい");
        vocab_store.add(&translation).unwrap();
        let different_foreign = Translation::new("no", "はい");
        match vocab_store.add(&different_foreign) {
            Err(VocabStoreError::DuplicateEntry) => {}
            Err(e) => assert!(
                false,
                "VocabStore did not return DuplicateEntry error: {:?}",
                e
            ),
            Ok(_) => assert!(false, "VocabStore did not return DuplicateEntry error"),
        }
    }

    #[test]
    fn test_add_duplicate_foreign() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        let vocab_store = VocabStore::init(&TEST_FILE).unwrap();
        let translation = Translation::new("yes", "はい");
        vocab_store.add(&translation).unwrap();
        let different_local = Translation::new("no", "はい");
        match vocab_store.add(&different_local) {
            Err(VocabStoreError::DuplicateEntry) => {}
            Err(e) => assert!(
                false,
                "VocabStore did not return DuplicateEntry error: {:?}",
                e
            ),
            Ok(_) => assert!(false, "VocabStore did not return DuplicateEntry error"),
        }
    }

    #[test]
    fn test_save() {
        use crate::schema::translations::dsl::*;

        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        let vocab_store = VocabStore::init(&TEST_FILE).unwrap();
        let mut translation = Translation::new("yes", "はい");
        vocab_store.add(&translation).unwrap();

        let conn = SqliteConnection::establish(&TEST_FILE).unwrap();
        let t: Translation = translations.load(&conn).unwrap().pop().unwrap();

        assert_eq!(t.guesses_foreign_total, 0);

        translation.guesses_foreign_total = 2;
        vocab_store.save(&translation).unwrap();

        let conn = SqliteConnection::establish(&TEST_FILE).unwrap();
        let t: Translation = translations.load(&conn).unwrap().pop().unwrap();

        assert_eq!(t.guesses_foreign_total, 2);
    }
}
