use crate::data_types::data_array::DataArray;
use crate::error_types::{CSVError, CSVErrorKind};
use crate::functions::convert::Convert;
use anyhow::{Error, Result};
use log::info;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

pub fn import_csv_data(
    file_path: &Path,
    has_headers: Option<bool>,
    delimiter: Option<u8>,
) -> Result<CSVData, Error> {
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
    let mut column_count: usize = 0;

    for result in reader.records() {
        let string_record = result?;
        sample_data.row_length = string_record.len();
        column_count += 1;
        for string in string_record.iter() {
            sample_data.data.push(string.to_string().trim().to_string()); // trim in case of whitespace
        }
    }
    sample_data.column_count = column_count;
    Ok(sample_data)
}

#[derive(Default, Debug)]
pub struct CSVData {
    pub data: Vec<String>,
    pub row_length: usize,
    pub column_count: usize,
}

impl CSVData {
    /// Retrieves a single datum from SampleData's vector as if it were a 2D array.
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
        let extracted_string = &self.data[self.row_length * (row - one) + (column - one)];
        T::from_str(extracted_string)
            .map_err(|error| CSVErrorKind::DataExtraction { source: error })
            .map_err(|error| CSVError {
                row,
                column,
                value: String::from(extracted_string),
                kind: error,
            })
    }

    /// Retrieves a column of data from CSVData's data vector.
    /// To imitate CSV row and column indexing, this function allows an option of
    /// indexing at 1 (it indexes from 0 as default).
    pub fn get_col<T>(
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

        for i in initial_index..self.column_count + initial_index {
            col.push(self.get_datum::<T>(i, column, one_based_index)?)
        }
        Ok(col)
    }

    // Extracts a column out of the CSV data as a DataArray object
    pub fn get_column_as_data_array<T>(
        &self,
        data_name: String,
        column: usize,
        one_based_index: bool,
        population: bool,
    ) -> Result<DataArray, Error>
    where
        T: FromStr + Clone + Copy + Debug + 'static,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
        f64: Convert<T>,
    {
        Ok(DataArray::new(
            data_name,
            self.get_col::<T>(column, Some(one_based_index))
                .map_err(|error| <CSVError<T>>::from(error))?,
            Some(population),
        )?)
    }
}
