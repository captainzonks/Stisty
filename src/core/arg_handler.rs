use crate::core::menu::main_menu;
use crate::data_types::csv::{import_csv_data, CSVData};
use anyhow::{Error, Result};
use clap::builder::PossibleValuesParser;
use clap::{
    arg, command, value_parser, Arg, ArgAction, ArgMatches, Args, Command, Parser, Subcommand,
    ValueEnum,
};
use log::info;
use std::path::PathBuf;
/////// BUILDER

pub fn generate_cli() -> Result<ArgMatches, Error> {
    let matches = command!()
        .subcommand_required(false)
        .arg_required_else_help(true)
        .arg(
            Arg::new("Menu")
                .short('m')
                .long("menu")
                .help("Run Stisty with an interactive CLI menu")
                .required(false)
                .exclusive(true)
                .action(ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("Configure")
                .short_flag('C')
                .long_flag("config")
                .about("Configure Stisty for command line use")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .arg(
                    Arg::new("csv-file")
                        .short('c')
                        .long("csv")
                        .help("Enter the path to the CSV file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set),
                )
                .subcommands([
                    Command::new("Single Sample t Test")
                        .short_flag('S')
                        .long_flag("single-sample")
                        .about("Run Single Sample t Test")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("column")
                                .short('c')
                                .long("column")
                                .help("Provide one column number for data extraction (0-based index)")
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Set),
                        ), // .arg(
                    //     Arg::new("data type")
                    //         .short('t')
                    //         .long("type")
                    //         .help("Enter if data in column is categorical or continuous")
                    //         .required(true)
                    //         .value_parser(PossibleValuesParser::new([
                    //             "Continuous",
                    //             "Categorical",
                    //         ])),
                    // ),
                    Command::new("Paired Samples t Test")
                        .short_flag('P')
                        .long_flag("paired-samples")
                        .about("Run Paired Samples t Test")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("columns")
                                .short('c')
                                .long("columns")
                                .help("Provide two column numbers for data extraction (0-based index)")
                                .required(true)
                                .num_args(2)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Append),
                        ),
                ]),
        )
        .get_matches();

    Ok(matches)
}

pub fn process_cli(matches: ArgMatches) -> Result<(), Error> {
    if let menu_mode = matches.get_flag("Menu") {
        if menu_mode {
            info!("Starting menu mode operation of Stisty...");
            main_menu().expect("Main menu failed");
            return Ok(());
        }
    }
    if let Some(matches) = matches.subcommand_matches("Configure") {
        if let Some(csv_file_path_buf) = matches.get_one::<PathBuf>("csv-file") {
            if csv_file_path_buf.as_path().is_file() {
                info!("Path is good; CSV file found!");
                info!("Importing CSV...");
                // let csv_data = import_csv_data(csv_file_path_buf.as_path(), None, None)?;
            }
        }
    }

    Ok(())
}

pub fn run_cli() -> Result<(), Error> {
    process_cli(generate_cli()?)?;
    Ok(())
}

/////// DERIVE

pub fn process_args() -> Result<(), Error> {
    info!("Processing arguments...");

    let args = Config::parse();

    if args.menu == true {
        info!("Starting menu mode operation of Stisty...");
        main_menu().expect("Main menu failed");
        return Ok(());
    }

    // let test_name = args.test_name;
    let csv_file_path_buff = args.config.csv_file;

    let csv_data: CSVData;

    if csv_file_path_buff.as_path().is_file() {
        info!("Path is good; CSV file found!");
        info!("Importing CSV...");
        csv_data = import_csv_data(csv_file_path_buff.as_path(), None, None)?;
    }
    Ok(())
}

#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Config {
    // Run Stisty in menu mode
    #[arg(short = 'm', long = "menu", exclusive = true, default_value_t = false)]
    menu: bool,

    #[command(flatten)]
    config: StistyConfig,
}

#[derive(Args)]
#[group(required = false, multiple = true)]
struct StistyConfig {
    // Path to CSV file
    #[arg(short = 'c', long = "csv-file")]
    csv_file: PathBuf,
}
