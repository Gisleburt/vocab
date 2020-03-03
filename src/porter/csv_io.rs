use std::io;

use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

use crate::porter::ExporterError;

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

impl From<Translation> for crate::Translation {
    fn from(t: Translation) -> Self {
        crate::Translation {
            local: t.local,
            foreign: t.foreign,
            guesses_local_total: t.guesses_local_total,
            guesses_local_correct: t.guesses_local_correct,
            guesses_foreign_total: t.guesses_foreign_total,
            guesses_foreign_correct: t.guesses_foreign_correct,
        }
    }
}

pub struct CsvReader<R: io::Read> {
    reader: Reader<R>,
}

impl<R: io::Read> CsvReader<R> {
    pub fn new(source: R) -> CsvReader<R> {
        CsvReader {
            reader: Reader::from_reader(source),
        }
    }
}

impl<R: io::Read> Iterator for CsvReader<R> {
    type Item = Result<crate::Translation, ExporterError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader
            .deserialize::<Translation>()
            .next()
            .map(|res| res.map(|rec| rec.into()).map_err(|e| e.into()))
    }
}

pub struct CsvWriter<W: io::Write> {
    writer: Writer<W>,
}

impl<W: io::Write> CsvWriter<W> {
    pub fn new(destination: W) -> CsvWriter<W> {
        CsvWriter {
            writer: Writer::from_writer(destination),
        }
    }

    pub fn write(&mut self, translation: crate::Translation) -> Result<(), ExporterError> {
        let csv_translation: Translation = translation.into();
        self.writer.serialize(csv_translation)?;
        self.writer.flush()?;
        Ok(())
    }
}
