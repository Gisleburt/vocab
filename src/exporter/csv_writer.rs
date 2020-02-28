use std::io;

use csv::Writer;
use serde::{Deserialize, Serialize};

use crate::exporter::ExporterError;

#[derive(Deserialize, Serialize)]
struct Translation {
    pub local: String,
    pub foreign: String,
    pub guesses_local_total: i32,
    pub guesses_local_correct: i32,
    pub guesses_foreign_total: i32,
    pub guesses_foreign_correct: i32,
}

impl From<crate::Translation> for Translation {
    fn from(t: crate::Translation) -> Self {
        Translation {
            local: t.local,
            foreign: t.foreign,
            guesses_local_total: t.guesses_local_total,
            guesses_local_correct: t.guesses_local_correct,
            guesses_foreign_total: t.guesses_foreign_total,
            guesses_foreign_correct: t.guesses_foreign_correct,
        }
    }
}

pub struct CsvWriter<W: io::Write> {
    writer: Writer<W>,
}

impl<W: io::Write> CsvWriter<W> {
    pub fn new(destination: W) -> Result<CsvWriter<W>, ExporterError> {
        let writer = csv::Writer::from_writer(destination);
        Ok(CsvWriter { writer })
    }

    pub fn write(&mut self, translation: crate::Translation) -> Result<(), ExporterError> {
        let csv_translation: Translation = translation.into();
        self.writer.serialize(csv_translation)?;
        self.writer.flush()?;
        Ok(())
    }
}
