use csv;
pub use csv_io::{CsvReader, CsvWriter};
use std::io;

mod csv_io;

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
