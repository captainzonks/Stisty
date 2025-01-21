use crate::core::error_types::{CSVError, CSVErrorKind};
use anyhow::{Error, Result};
use log::{debug, info};
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

pub fn import_csv_data(
    file_path: &Path,
    has_headers: Option<bool>,
    delimiter: Option<u8>,
) -> Result<CSVData, Error> {
    info!("Importing CSV...");

    let mut reader_builder = csv::ReaderBuilder::new();

    match has_headers {
        Some(has_headers) => reader_builder.has_headers(has_headers),
        _ => reader_builder.has_headers(true),
    };
    match delimiter {
        Some(delimiter) => reader_builder.delimiter(delimiter),
        _ => reader_builder.delimiter(b','),
    };

    let mut reader = reader_builder.from_path(file_path)?;

    let mut sample_data: CSVData = Default::default();
    sample_data.headers = reader.headers()?.clone().iter().map(String::from).collect();
    let mut column_count: usize = 0;

    for result in reader.records() {
        let string_record = result?;
        sample_data.total_columns = string_record.len();
        column_count += 1;
        for string in string_record.iter() {
            sample_data.data.push(string.to_string().trim().to_string()); // trim in case of whitespace
        }
    }
    sample_data.total_rows = column_count;

    info!("CSV successfully imported!");
    Ok(sample_data)
}

#[derive(Default, Debug, Clone)]
pub struct CSVData {
    pub data: Vec<String>,
    pub headers: Vec<String>,
    pub total_columns: usize,
    pub total_rows: usize,
}

impl CSVData {
    pub fn new(
        data: Vec<String>,
        headers: Vec<String>,
        total_columns: usize,
        total_rows: usize,
    ) -> CSVData {
        CSVData {
            data,
            headers,
            total_columns,
            total_rows,
        }
    }

    /// Retrieves a column of data from CSVData's data vector.
    /// 0-Based Indexing by default, however it allows the option to use 1 instead.
    pub fn get_column<T>(
        &self,
        column: usize,
        one_based_index: Option<bool>,
    ) -> Result<Vec<T>, CSVError<T>>
    where
        T: FromStr + Clone + Debug,
    {
        info!(
            "Retrieving column {} from CSV using {}-based indexing",
            column,
            if one_based_index.unwrap_or_default() {
                1
            } else {
                0
            }
        );
        let initial_index: usize = if one_based_index.unwrap_or_default() {
            1
        } else {
            0
        };

        let mut col: Vec<T> = Vec::with_capacity(self.data.len());

        for i in initial_index..self.total_rows + initial_index {
            col.push(self.get_datum::<T>(i, column, one_based_index)?)
        }
        Ok(col)
    }

    /// Retrieves a single datum from CSVData's data vector as if it were a 2D array.
    /// To imitate CSV row and column indexing, this function allows an option of
    /// indexing at 1 (it indexes from 0 as default).
    pub fn get_datum<T>(
        &self,
        row: usize,
        column: usize,
        one_based_index: Option<bool>,
    ) -> Result<T, CSVError<T>>
    where
        T: FromStr + Clone + Debug,
    {
        let one: usize = if one_based_index.unwrap_or_default() {
            1
        } else {
            0
        };
        // row_len * row + column (row major)
        let extracted_string = &self.data[self.total_columns * (row - one) + (column - one)];

        T::from_str(extracted_string)
            .map_err(|error| CSVErrorKind::DataExtraction { source: error })
            .map_err(|error| CSVError {
                row,
                column,
                value: String::from(extracted_string),
                kind: error,
            })
    }
}

pub(crate) fn generate_dummy_csv() -> CSVData {
    CSVData::new(
        String::from("1,15,CO,9,3,2,27,MI,7,2,3,18,NY,6,5")
            .split(',')
            .map(|s| s.to_string())
            .collect(),
        String::from("Participant,Age,State,Stress Before Exam,Stress After Exam")
            .split(',')
            .map(|s| s.to_string())
            .collect(),
        5,
        3,
    )
}

#[cfg(test)]
mod tests {
    use super::{generate_dummy_csv, import_csv_data};
    use std::path::Path;

    #[test]
    fn csv_data_is_ok() {
        let test_data_path: &Path = Path::new("./csv-files/test_data.csv");
        let csv_import_result = import_csv_data(test_data_path, None, None);
        assert!(csv_import_result.is_ok());
    }

    #[test]
    fn continuous_data_column_is_ok() {
        let extracted_numerical_column_result = &generate_dummy_csv().get_column::<i32>(1, None);
        assert!(extracted_numerical_column_result.is_ok());
    }

    #[test]
    fn categorical_data_column_is_ok() {
        let extracted_string_column_result = &generate_dummy_csv().get_column::<String>(2, None);
        assert!(extracted_string_column_result.is_ok());
    }

    #[test]
    fn string_datum_is_ok() {
        let extracted_string_datum_result = &generate_dummy_csv().get_datum::<String>(2, 2, None);
        assert!(extracted_string_datum_result.is_ok());
    }

    #[test]
    fn number_datum_is_ok() {
        let extracted_numerical_datum_result = &generate_dummy_csv().get_datum::<i32>(2, 1, None);
        assert!(extracted_numerical_datum_result.is_ok());
    }
}
