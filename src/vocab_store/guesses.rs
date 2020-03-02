use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{schema::RANDOM, Guess, Translation, VocabStoreError};

pub struct Guesses<'a> {
    conn: &'a SqliteConnection,
}

impl<'a> Guesses<'a> {
    pub fn new(conn: &'a SqliteConnection) -> Guesses<'a> {
        Guesses { conn }
    }
}

impl<'a> Iterator for Guesses<'a> {
    type Item = Result<Guess, VocabStoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        use crate::schema::translations::dsl::*;

        let query_result = translations
            .order(RANDOM)
            .limit(1)
            .load::<Translation>(self.conn);

        match query_result {
            Err(e) => Some(Err(e.into())),
            Ok(mut translation_results) => translation_results.pop().map(|t: Translation| {
                Ok(if t.guesses_foreign_correct > t.guesses_local_correct {
                    Guess::Local(t)
                } else {
                    Guess::Foreign(t)
                })
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::ops::Deref;

    use diesel::{Connection, RunQueryDsl, SqliteConnection};

    use crate::vocab_store::guesses::Guesses;
    use crate::{Translation, VocabStore};

    const TEST_FILE: &str = "test.sqlite";

    #[test]
    fn test_gets_a_translation() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        let _ = VocabStore::init(&TEST_FILE).unwrap();

        let conn = SqliteConnection::establish(&TEST_FILE).unwrap();
        let mut guesses = Guesses::new(&conn);
        let translation = Translation::new("yes", "はい");
        diesel::insert_into(crate::schema::translations::table)
            .values(&translation)
            .execute(&conn)
            .unwrap();

        assert_eq!(guesses.next().unwrap().unwrap().deref(), &translation);
        assert_eq!(guesses.next().unwrap().unwrap().deref(), &translation);
    }
}
