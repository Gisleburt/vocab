use csv;
pub use csv_writer::CsvWriter;
pub use db_reader::DbReader;
use std::io;

mod csv_writer;
mod db_reader;

#[derive(Debug)]
pub enum ExporterError {
    CsvError(csv::Error),
    IoError(io::Error),
}

impl From<csv::Error> for ExporterError {
    fn from(e: csv::Error) -> Self {
        ExporterError::CsvError(e)
    }
}

impl From<io::Error> for ExporterError {
    fn from(e: io::Error) -> Self {
        ExporterError::IoError(e)
    }
}
