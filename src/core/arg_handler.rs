use crate::core::menu::main_menu;
use crate::data_types::csv::{import_csv_data, CSVData};
use crate::data_types::statistics::{
    run_anova_test, run_independent_groups_t_test, run_paired_samples_t_test,
    run_single_sample_t_test,
};

use anyhow::{anyhow, Error, Result};
use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command};
use log::info;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct DescriptionConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default, Clone)]
pub struct SingleSampleTConfig {
    pub csv_data: CSVData,
    pub description_config: Option<DescriptionConfig>,
    pub column_index: usize,
    pub mu: f64,
}

#[derive(Debug, Default, Clone)]
pub struct PairedSamplesTConfig {
    pub csv_data: CSVData,
    pub description_config: Option<DescriptionConfig>,
    pub column_indices: Vec<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct IndependentGroupsTConfig {
    pub csv_data: CSVData,
    pub description_config: Option<DescriptionConfig>,
    pub categorical_column_index: usize,
    pub continuous_column_index: usize,
}

#[derive(Debug, Default, Clone)]
pub struct ANOVAConfig {
    pub csv_data: CSVData,
    pub description_config: Option<DescriptionConfig>,
    pub categorical_column_index: usize,
    pub continuous_column_index: usize,
}

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
                .args([
                    Arg::new("csv-file")
                        .short('c')
                        .long("csv")
                        .help("Enter the path to the CSV file")
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                        .action(ArgAction::Set),
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .help("Name of statistic and file export")
                        .long_help("Name of the statistic to be used for logging and file export")
                        .required(false)
                        .value_parser(value_parser!(String)),
                    Arg::new("description")
                        .short('d')
                        .long("description")
                        .help("Description of statistic")
                        .long_help(
                            "Description of statistic to be used for logging and file \
                        export",
                        )
                        .requires("name")
                        .value_parser(value_parser!(String)),
                ])
                .subcommands([
                    Command::new("Single Sample t Test")
                        .short_flag('S')
                        .long_flag("single-sample")
                        .about("Run Single Sample t Test")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("column")
                                .short('c')
                                .long("column")
                                .help("CSV column index of continuous data (0-based index)")
                                .long_help(
                                    "Provide a single column index for data extraction \
                                (0-based index). Data must be continuous.",
                                )
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Set),
                            Arg::new("mu")
                                .short('m')
                                .long("mu")
                                .help("Population mean")
                                .long_help(
                                    "Population mean to which the sample's mean will be \
                                compared.",
                                )
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(f64))
                                .action(ArgAction::Set),
                        ]),
                    Command::new("Paired Samples t Test")
                        .short_flag('P')
                        .long_flag("paired-samples")
                        .about("Run Paired Samples t Test")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("columns")
                                .short('c')
                                .long("columns")
                                .help("Two CSV column indices of continuous data (0-based index)")
                                .long_help(
                                    "Provide two column indices for data extraction \
                                (0-based index). They must be continuous data and consist of \
                                identical row counts.",
                                )
                                .required(true)
                                .num_args(2)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Append),
                        ),
                    Command::new("Independent Groups t Test")
                        .short_flag('I')
                        .long_flag("ind-groups")
                        .about("Run Independent Groups t Test")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("nominal")
                                .short('n')
                                .long("nominal")
                                .help(
                                    "A CSV column index of categorical data (0-based index, 2 \
                                levels)",
                                )
                                .long_help(
                                    "Provide a column index for data extraction (0-based \
                                index). They must be categorical data and consist of exactly 2 \
                                levels.",
                                )
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Set),
                            Arg::new("continuous")
                                .short('c')
                                .long("continuous")
                                .help("A CSV column index of continuous data (0-based index)")
                                .long_help(
                                    "Provide a column index for data extraction (0-based \
                            index). They must be continuous data and align to the provided \
                            categorical column in expected row indices.",
                                )
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Set),
                        ]),
                    Command::new("ANOVA")
                        .short_flag('A')
                        .long_flag("ANOVA")
                        .about("Run ANOVA Test")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("nominal")
                                .short('n')
                                .long("nominal")
                                .help(
                                    "A CSV column index of categorical data (0-based index, 3 or \
                                    more levels)",
                                )
                                .long_help(
                                    "Provide a column index for data extraction (0-based \
                                index). They must be categorical data and consist of 3 or more \
                                levels.",
                                )
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Set),
                            Arg::new("continuous")
                                .short('c')
                                .long("continuous")
                                .help("A CSV column index of continuous data (0-based index)")
                                .long_help(
                                    "Provide a column index for data extraction (0-based \
                            index). They must be continuous data and align to the provided \
                            categorical column in expected row indices.",
                                )
                                .required(true)
                                .num_args(1)
                                .value_parser(value_parser!(usize))
                                .action(ArgAction::Set),
                        ]),
                ]),
        )
        .get_matches();

    Ok(matches)
}

pub fn process_cli(matches: ArgMatches) -> Result<(), Error> {
    let menu_mode = matches.get_flag("Menu");
    if menu_mode {
        info!("Starting menu mode operation of Stisty...");
        main_menu()?;
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("Configure") {
        let mut new_csv_data: CSVData = CSVData::default();
        if let Some(csv_file_path_buf) = matches.get_one::<PathBuf>("csv-file") {
            if csv_file_path_buf.as_path().is_file() {
                info!("Path is good; CSV file found!");
                new_csv_data = import_csv_data(csv_file_path_buf.as_path(), Some(true), None)?;

                let mut file_name: String = String::new();
                if let Some(name_arg) = matches.get_one::<String>("name") {
                    info!("Found name '{}' to be used for file export", name_arg);
                    file_name = name_arg.clone();
                }
                let mut description: String = String::new();
                if let Some(description_arg) = matches.get_one::<String>("description") {
                    info!(
                        "Found description '{}' to be used for file export",
                        description_arg
                    );
                    description = description_arg.clone();
                }

                let mut new_description_config: DescriptionConfig = DescriptionConfig::default();
                if !file_name.is_empty() {
                    new_description_config = DescriptionConfig {
                        name: file_name.clone(),
                        description: description.clone(),
                    }
                }

                fn get_categorical_continuous_column_indices(
                    arg_matches: &ArgMatches,
                ) -> Result<(usize, usize), Error> {
                    let categorical_column_index_option = arg_matches.get_one::<usize>("nominal");
                    let continuous_column_index_option = arg_matches.get_one::<usize>("continuous");

                    let categorical_column_index;
                    let continuous_column_index;
                    match categorical_column_index_option {
                        None => return Err(anyhow!("Bad categorical column index")),
                        Some(index) => categorical_column_index = *index,
                    }
                    match continuous_column_index_option {
                        None => return Err(anyhow!("Bad continuous column index")),
                        Some(index) => continuous_column_index = *index,
                    }

                    Ok((categorical_column_index, continuous_column_index))
                }

                match matches.subcommand() {
                    None => {
                        return Err(anyhow!("No subcommand found!"));
                    }
                    Some(("Single Sample t Test", arg_matches)) => {
                        let column_index_option = arg_matches.get_one::<usize>("column");
                        let mu_option = arg_matches.get_one::<f64>("mu");
                        let mut column_index_arg: usize = 0;
                        let mut mu_arg: f64 = 0.0;
                        match column_index_option {
                            None => return Err(anyhow!("Bad column index")),
                            Some(index) => {
                                column_index_arg = *index;
                            }
                        }
                        match mu_option {
                            None => return Err(anyhow!("Bad mu")),
                            Some(mu) => {
                                mu_arg = *mu;
                            }
                        }

                        let single_sample_t_config = SingleSampleTConfig {
                            csv_data: new_csv_data,
                            description_config: Some(new_description_config),
                            column_index: column_index_arg,
                            mu: mu_arg,
                        };

                        run_single_sample_t_test(single_sample_t_config)?;
                        return Ok(());
                    }
                    Some(("Paired Samples t Test", arg_matches)) => {
                        let column_indices_option = arg_matches.get_many::<usize>("columns");
                        let mut column_indices_arg = vec![];
                        match column_indices_option {
                            None => return Err(anyhow!("Bad column indices")),
                            Some(indices) => {
                                column_indices_arg = indices.map(|x| *x).collect();
                            }
                        }

                        let paired_samples_t_config = PairedSamplesTConfig {
                            csv_data: new_csv_data,
                            description_config: Some(new_description_config),
                            column_indices: column_indices_arg,
                        };

                        run_paired_samples_t_test(paired_samples_t_config)?;
                        return Ok(());
                    }
                    Some(("Independent Groups t Test", arg_matches)) => {
                        let indices_tuple = get_categorical_continuous_column_indices(arg_matches)?;

                        let independent_groups_t_config = IndependentGroupsTConfig {
                            csv_data: new_csv_data,
                            description_config: Some(new_description_config),
                            categorical_column_index: indices_tuple.0,
                            continuous_column_index: indices_tuple.1,
                        };

                        run_independent_groups_t_test(independent_groups_t_config)?;
                        return Ok(());
                    }
                    Some(("ANOVA", arg_matches)) => {
                        let indices_tuple = get_categorical_continuous_column_indices(arg_matches)?;

                        let anova_config = ANOVAConfig {
                            csv_data: new_csv_data,
                            description_config: Some(new_description_config),
                            categorical_column_index: indices_tuple.0,
                            continuous_column_index: indices_tuple.1,
                        };

                        run_anova_test(anova_config)?;
                        return Ok(());
                    }
                    _ => {}
                }
            } else {
                info!("CSV file not found");
                return Err(anyhow!(
                    "CSV file not found at {}",
                    csv_file_path_buf.as_path().display()
                ));
            }
        }
    }

    Ok(())
}

pub fn run_cli() -> Result<(), Error> {
    process_cli(generate_cli()?)?;
    Ok(())
}
