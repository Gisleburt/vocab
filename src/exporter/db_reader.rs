use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

use crate::schema::RANDOM;
use crate::{Translation, VocabStoreError};

pub struct DbReader<'a> {
    conn: &'a SqliteConnection,
    page: i64,
}

impl<'a> DbReader<'a> {
    pub fn new(conn: &'a SqliteConnection) -> DbReader<'a> {
        DbReader { conn, page: 0 }
    }
}

impl<'a> Iterator for DbReader<'a> {
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
            Ok(mut translation_results) => translation_results.pop().map(|t: Translation| Ok(t)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use diesel::{Connection, RunQueryDsl, SqliteConnection};

    use crate::exporter::Exporter;
    use crate::{Translation, VocabStore};

    const TEST_FILE: &str = "test.sqlite";

    #[test]
    fn test_exports_all_translation() {
        let _ = fs::remove_file(&TEST_FILE); // Ok if it fails;
        VocabStore::init(&TEST_FILE).unwrap(); // Init DB

        let conn = SqliteConnection::establish(&TEST_FILE).unwrap();
        let mut exporter = Exporter::new(&conn);
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
