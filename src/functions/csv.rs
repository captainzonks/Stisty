use std::any::type_name;
use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;
use log::{error, info};
use anyhow::{anyhow, Context, Result};
use crate::error_types::{CSVError, CSVErrorKind};

pub fn import_csv_data(file_path: &Path, has_headers: Option<bool>, delimiter: Option<u8>) -> Result<CSVData, anyhow::Error> {
    let mut reader_builder = csv::ReaderBuilder::new();

    match has_headers {
        Some(has_headers) => reader_builder.has_headers(has_headers),
        _ => reader_builder.has_headers(true),
    };
    match delimiter {
        Some(delimiter) => reader_builder.delimiter(delimiter),
        _ => reader_builder.delimiter(b',')
    };

    let mut reader = reader_builder.from_path(file_path)?;

    let mut sample_data: CSVData = Default::default();
    let mut column_count: usize = 0;

    for result in reader.records() {
        match result {
            Ok(string_record) => {
                sample_data.row_length = string_record.len();
                column_count += 1;
                for string in string_record.iter() {
                    sample_data.data.push(string.to_string().trim().to_string()); // trim in case of whitespace
                }
            }
            Err(err) => {
                error!("{}", err);
                return Err(anyhow!("{}", err));
            }
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
    pub fn get_datum<T>(&self, row: usize, column: usize, one_based_index: Option<bool>) -> Result<T, CSVError<T>>
    where
        T: FromStr + Clone + Debug,
    {
        let one: usize = if one_based_index.unwrap_or_default() { 1 } else { 0 };
        // row_len * row + column (row major)
        let extracted_string = &self.data[self.row_length * (row - one) + (column - one)];
        extracted_string.parse::<T>()?.clone()
        // match extracted_string.parse::<T>().clone() {
        //     Ok(val) => Ok(val),
        //     Err(err) => {
        //         error!("datum \"{}\" could not be parsed into {} type", extracted_string, type_name::<T>());
        //         Err(err)
        //     }
        // }
        // if index < self.data.len() {
        //     let extracted_string = &self.data[self.row_length * (row - one) + (column - one)];
        //     match extracted_string.parse::<T>().clone() {
        //         Ok(val) => Ok(val),
        //         Err(_) => Err(format!("Could not convert {} to number", extracted_string))
        //     }
        //
        //     // .unwrap_or_else(|err| {
        //     // error!("Error parsing {} from row {} and column {}; Error: {:?}",
        //     //     extracted_string,
        //     //     row,
        //     //     column,
        //     //     err
        //     // );
        //     // panic!(
        //     //     "Error parsing {},\nError: {:?}",
        //     //     extracted_string,
        //     //     err
        //     // )
        // } else {
        //     Err()
        // }

        // converted
        // } else {
        //     panic!("Error: index {} is greater than data's vector length ({})", index, self.data.len());
        // }
    }

    /// Retrieves a column of data from CSVData's data vector.
    /// To imitate CSV row and column indexing, this function allows an option of
    /// indexing at 1 (it indexes from 0 as default).
    pub fn get_col<T>(&self, column: usize, one_based_index: Option<bool>) -> Result<Vec<T>, CSVError<T>>
    where
        T: FromStr + Clone + Debug,
    {
        info!("Retrieving column {} from CSV using {}-based indexing", column, if one_based_index.unwrap_or_default() { 1 } else { 0 });
        let one: usize = if one_based_index.unwrap_or_default() { 1 } else { 0 };
        let mut col: Vec<T> = Vec::with_capacity(self.data.len());

        for i in one..self.column_count + one {
            if let datum = self.get_datum::<T>(i, column, one_based_index) {
                match datum {
                    Ok(val) => col.push(val),
                    Err(ref err) => { datum?; }
                }
                // col.push(datum?);
            }
        }
        Ok(col)
    }
}