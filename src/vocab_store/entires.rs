use crate::{Translation, VocabStoreError};
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

pub struct Entries<'c> {
    conn: &'c SqliteConnection,
    page: i64,
}

impl<'c> Entries<'c> {
    pub fn new(conn: &'c SqliteConnection) -> Entries<'c> {
        Entries { conn, page: 0 }
    }
}

impl<'c> Iterator for Entries<'c> {
    type Item = Result<Translation, VocabStoreError>;

    fn next(&mut self) -> Option<Self::Item> {
        use crate::schema::translations::dsl::*;

        let query_result = translations
            .limit(1)
            .offset(self.page)
            .load::<Translation>(self.conn);

        self.page += 1;

        match query_result {
            Err(e) => Some(Err(e.into())),
            Ok(mut translation_results) => translation_results.pop().map(Ok),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Entries;
    use crate::{Translation, VocabStore};
    use diesel::{Connection, RunQueryDsl, SqliteConnection};
    use std::fs;

    const TEST_FILE: &str = "test.sqlite";

    #[test]
    fn test_exports_all_translation() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        let vocab_store = VocabStore::init(&TEST_FILE).unwrap(); // Init DB

        let conn = SqliteConnection::establish(&TEST_FILE).unwrap();
        let mut exporter = Entries::new(&conn);
        let translation_yes = Translation::new("yes", "はい");
        let translation_no = Translation::new("no", "いいえ");
        diesel::insert_into(crate::schema::translations::table)
            .values(&translation_yes)
            .execute(&conn)
            .unwrap();
        diesel::insert_into(crate::schema::translations::table)
            .values(&translation_no)
            .execute(&conn)
            .unwrap();

        assert_eq!(exporter.next().unwrap().unwrap(), translation_yes);
        assert_eq!(exporter.next().unwrap().unwrap(), translation_no);
        assert!(exporter.next().is_none());
    }
}
