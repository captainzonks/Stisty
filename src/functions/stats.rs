// use std::error::Error;
use anyhow::{Error, Result};
use std::fmt::Debug;
use std::str::FromStr;
use crate::data_types::data_array::DataArray;
use crate::error_types::CSVError;
use crate::functions::convert::Convert;
use crate::functions::csv::CSVData;

pub fn get_data_stats<T>(csv_data: &CSVData, data_name: String, column: usize, one_based_index: bool, population: bool) -> Result<DataArray, Error>
where
    T: FromStr + Clone + Copy + Debug + 'static,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    f64: Convert<T>,
{
    Ok(DataArray::new(data_name, csv_data.get_col::<T>(column, Some(one_based_index))
        .map_err(|error| <CSVError<T>>::from(error))?, Some(population))?)
}