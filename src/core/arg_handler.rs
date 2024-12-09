use crate::data_types::csv::{import_csv_data, CSVData};
use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::info;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Config {
    // Path to CSV file for statistical analysis
    #[arg(short = 'p', long = "path")]
    csv_file_path: PathBuf,

    #[command(subcommand)]
    statistic: Statistic,
}

#[derive(Subcommand, Clone, Debug)]
enum Statistic {
    // Choose a statistic to run
    Statistic(StatisticConfig),
}

#[derive(Args, Clone, Debug)]
struct StatisticConfig {
    // Statistic to run
    statistic_type: StatisticType,
}

#[derive(Clone, Debug, ValueEnum)]
enum DataType {
    // Continuous
    C,
    // Nominal
    N,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum StatisticType {
    SingleSampleT,
    PairedSampleT,
    IndependentGroupsT,
    ZTest,
    OneWayANOVA,
}

pub fn process_args() -> Result<(), Error> {
    info!("Processing arguments...");

    let args = Config::parse();

    let csv_file_path_buff = args.csv_file_path;
    // let test_name = args.test_name;
    let statistic = args.statistic;

    let csv_data: CSVData;

    if csv_file_path_buff.as_path().is_file() {
        info!("Path is good; CSV file found!");
        info!("Importing CSV...");
        csv_data = import_csv_data(csv_file_path_buff.as_path(), None, None)?;
    }

    // if test_name.is_some() {
    //     info!("test_name: {}", test_name.unwrap());
    // }

    info!("Statistic: {:?}", statistic);

    Ok(())
}
