use crate::data_types::csv::{import_csv_data, CSVData};
use crate::data_types::data_array::{CategoricalDataArray, ContinuousDataArray};
use crate::data_types::statistics::{PairedSamplesT, ANOVA};
use anyhow::{anyhow, Error, Result};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomType, CustomUserError, Select, Text,
};
use log::info;
use std::env;
use std::io::ErrorKind;
use std::path::Path;

pub fn main_menu() -> Result<(), Error> {
    let statistics = vec![
        "Single Sample T",
        "Paired Samples T",
        "Independent Groups T",
        "One Way ANOVA",
    ];

    let current_dir = env::current_dir()?;
    let help_message = format!("Current directory: {}", current_dir.to_string_lossy());

    let csv_file_path_string =
        Text::new("Please enter the path and filename of the CSV you'd like to load:")
            .with_autocomplete(FilePathCompleter::new()?)
            .with_help_message(&help_message)
            .prompt();

    let mut csv_data: CSVData = CSVData::default();

    match csv_file_path_string {
        Ok(path) => {
            let path = Path::new(&path);
            if path.is_file() {
                info!("Path is good; CSV file found!");
                info!(
                    "Importing CSV...{}",
                    path.file_name().unwrap().to_string_lossy()
                );
                csv_data = import_csv_data(path, None, None)?;
            }
        }
        Err(error) => println!("There was an error retrieving the path: {error:?}"),
    }

    let statistic = Select::new("What statistic would you like to run?", statistics).prompt()?;

    match statistic {
        "Single Sample T" => single_sample_t_menu(&csv_data)?,
        "Paired Samples T" => paired_samples_t_menu(&csv_data)?,
        "Independent Groups T" => independent_groups_t_menu(&csv_data)?,
        "One Way ANOVA" => one_way_anova_menu(&csv_data)?,
        &_ => {}
    }

    Ok(())
}

fn single_sample_t_menu(csv_data: &CSVData) -> Result<(), Error> {
    let headers = csv_data.headers.clone();
    let column_header = Select::new(
        "Please select a column of continuous data as the dependent variable:",
        headers.clone(),
    )
    .prompt()?;
    let mu =
        CustomType::<f64>::new("Please enter the population's mean (mu) for the test:").prompt()?;

    let column_index_opt = headers.iter().position(|x| column_header.eq(x));
    let column_index: usize;
    match column_index_opt {
        Some(index) => column_index = index,
        None => return Err(anyhow!("Error in getting column index")),
    }

    let column_data = csv_data.get_column::<f64>(column_index, None)?;
    let continuous_data_array = ContinuousDataArray::new(
        String::from("PLACEHOLDER"),
        &column_data,
        column_index,
        csv_data.headers[column_index].clone(),
        None,
    )?;

    let result = crate::data_types::statistics::SingleSampleT::new(
        String::from("PLACEHOLDER"),
        String::from("PLACEHOLDER"),
        &continuous_data_array,
        mu,
    )?;

    result.print();

    Ok(())
}

fn paired_samples_t_menu(csv_data: &CSVData) -> Result<(), Error> {
    let headers = csv_data.headers.clone();
    let column_header_x = Select::new(
        "Please select a column of continuous data as the first measurement:",
        headers.clone(),
    )
    .prompt()?;
    let column_header_y = Select::new(
        "Please select a column of continuous data as the second measurement:",
        headers.clone(),
    )
    .prompt()?;

    let mut column_index_option = headers.iter().position(|x| column_header_x.eq(x));
    let column_x_index: usize;
    match column_index_option {
        Some(index) => column_x_index = index,
        None => return Err(anyhow!("Error in getting first measurement column index")),
    }
    column_index_option = headers.iter().position(|y| column_header_y.eq(y));
    let column_y_index: usize;
    match column_index_option {
        Some(index) => column_y_index = index,
        None => return Err(anyhow!("Error in getting second measurement column index")),
    }

    let data_x = csv_data.get_column::<f64>(column_x_index, None)?;
    let data_y = csv_data.get_column::<f64>(column_y_index, None)?;

    let data_array_x = ContinuousDataArray::new(
        String::from("PLACEHOLDER"),
        &data_x,
        column_x_index,
        csv_data.headers[column_x_index].clone(),
        None,
    )?;
    let data_array_y = ContinuousDataArray::new(
        String::from("PLACEHOLDER"),
        &data_y,
        column_y_index,
        csv_data.headers[column_y_index].clone(),
        None,
    )?;

    let result = PairedSamplesT::new(
        String::from("PLACEHOLDER"),
        String::from("PLACEHOLDER"),
        &data_array_x,
        &data_array_y,
    )?;

    result.print();

    Ok(())
}

fn independent_groups_t_menu(csv_data: &CSVData) -> Result<(), Error> {
    let headers = csv_data.headers.clone();
    let categorical_column_header = Select::new(
        "Please select a column of categorical data with only two levels as the independent variable:",
        headers.clone(),
    )
    .prompt()?;

    let continuous_column_header = Select::new(
        "Please select a column of continuous data as the dependent variable:",
        headers.clone(),
    )
    .prompt()?;

    let categorical_column_index_opt = headers.iter().position(|x| categorical_column_header.eq(x));
    let categorical_column_index: usize;
    match categorical_column_index_opt {
        Some(index) => categorical_column_index = index,
        None => return Err(anyhow!("Error in getting categorical column index")),
    }

    let continuous_column_index_opt = headers.iter().position(|y| continuous_column_header.eq(y));
    let continuous_column_index: usize;
    match continuous_column_index_opt {
        Some(index) => continuous_column_index = index,
        None => return Err(anyhow!("Error in getting continuous column index")),
    }

    let categorical_column_data = csv_data.get_column::<String>(categorical_column_index, None)?;
    let categorical_data_array = CategoricalDataArray::new(
        String::from("PLACEHOLDER"),
        &categorical_column_data,
        categorical_column_index,
        csv_data.headers[categorical_column_index].clone(),
        None,
    )?;

    let continuous_column_data = csv_data.get_column::<f64>(continuous_column_index, None)?;
    let continuous_data_array = ContinuousDataArray::new(
        String::from("PLACEHOLDER"),
        &continuous_column_data,
        continuous_column_index,
        csv_data.headers[continuous_column_index].clone(),
        None,
    )?;

    let result = crate::data_types::statistics::IndependentGroupsT::new(
        String::from("PLACEHOLDER"),
        String::from("PLACEHOLDER"),
        &categorical_data_array,
        &continuous_data_array,
    )?;

    result.print();

    Ok(())
}

fn one_way_anova_menu(csv_data: &CSVData) -> Result<(), Error> {
    let headers = csv_data.headers.clone();
    let categorical_column_header = Select::new(
        "Please select a column of categorical data with three or more levels as the independent variable:",
        headers.clone(),
    )
    .prompt()?;

    let continuous_column_header = Select::new(
        "Please select a column of continuous data as the dependent variable:",
        headers.clone(),
    )
    .prompt()?;

    let categorical_column_index_opt = headers.iter().position(|x| categorical_column_header.eq(x));
    let categorical_column_index: usize;
    match categorical_column_index_opt {
        Some(index) => categorical_column_index = index,
        None => return Err(anyhow!("Error in getting categorical column index")),
    }

    let continuous_column_index_opt = headers.iter().position(|y| continuous_column_header.eq(y));
    let continuous_column_index: usize;
    match continuous_column_index_opt {
        Some(index) => continuous_column_index = index,
        None => return Err(anyhow!("Error in getting continuous column index")),
    }

    let categorical_column_data = csv_data.get_column::<String>(categorical_column_index, None)?;
    let categorical_data_array = CategoricalDataArray::new(
        String::from("PLACEHOLDER"),
        &categorical_column_data,
        categorical_column_index,
        csv_data.headers[categorical_column_index].clone(),
        None,
    )?;

    let continuous_column_data = csv_data.get_column::<f64>(continuous_column_index, None)?;
    let continuous_data_array = ContinuousDataArray::new(
        String::from("PLACEHOLDER"),
        &continuous_column_data,
        continuous_column_index,
        csv_data.headers[continuous_column_index].clone(),
        None,
    )?;

    let result = ANOVA::new(
        String::from("PLACEHOLDER"),
        String::from("PLACEHOLDER"),
        &categorical_data_array,
        &continuous_data_array,
        None,
    )?;

    result.print();

    Ok(())
}

#[derive(Clone, Default)]
pub struct FilePathCompleter {
    input: String,
    paths: Vec<String>,
    _os_slash: char,
    _opposite_slash: char,
}

impl FilePathCompleter {
    fn new() -> Result<FilePathCompleter, Error> {
        let new_file_path_completer = FilePathCompleter {
            input: "".to_string(),
            paths: vec![],
            _os_slash: if env::consts::OS == "windows" {
                '\\'
            } else {
                '/'
            },
            _opposite_slash: if env::consts::OS == "windows" {
                '/'
            } else {
                '\\'
            },
        };
        Ok(new_file_path_completer)
    }

    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input && !self.paths.is_empty() {
            // println!("INPUT: {:?}", input);
            // self.get_suggestions(input)?;
            return Ok(());
        }

        self.input = input
            .replace(self._opposite_slash, &*self._os_slash.to_string())
            .to_owned();
        // println!("self.input: {:?}", input);
        self.paths.clear();

        // println!("\nUPDATE INPUT: {}\n", self.input);

        let input_path = std::path::PathBuf::from(self.input.clone());

        let fallback_parent = input_path
            .parent()
            .map(|p| {
                if p.to_string_lossy() == "" {
                    // println!("1: {:?}", p);
                    std::path::PathBuf::from(".")
                } else {
                    // println!("2: {:?}", p);
                    p.to_owned()
                }
            })
            .unwrap_or_else(|| std::path::PathBuf::from("."));

        let scan_dir = if self.input.ends_with(self._os_slash) {
            // info!("Input path being used!");
            input_path
        } else {
            // info!("Falling back to {:?}", fallback_parent);
            fallback_parent.clone()
        };

        let entries = match std::fs::read_dir(scan_dir) {
            Ok(read_dir) => Ok(read_dir),
            Err(err) if err.kind() == ErrorKind::NotFound => std::fs::read_dir(fallback_parent),
            Err(err) => Err(err),
        }?
        .collect::<Result<Vec<_>, _>>()?;

        for entry in entries {
            let path = entry.path();
            let path_str = if path.is_dir() {
                format!("{}{}", path.to_string_lossy(), self._os_slash)
            } else {
                path.to_string_lossy().to_string()
            };
            self.paths.push(path_str);
        }

        Ok(())
    }

    fn fuzzy_sort(&self, input: &str) -> Vec<(String, i64)> {
        let cleaned_input = &*input.replace(self._opposite_slash, &*self._os_slash.to_string());
        // println!("\nFUZZY SORT CALLED! INPUT: {}\n", cleaned_input);
        // println!("{:#?}", self.paths);
        let mut matches: Vec<(String, i64)> = self
            .paths
            .iter()
            .filter_map(|path| {
                SkimMatcherV2::default()
                    .smart_case()
                    .fuzzy_match(path, cleaned_input)
                    .map(|score| (path.clone(), score))
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        // println!("{:#?}", matches);
        matches
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let cleaned_input = &*input.replace(self._opposite_slash, &*self._os_slash.to_string());

        // println!("\nGET SUGGESTIONS CALLED! INPUT: {}\n", cleaned_input);
        self.update_input(cleaned_input)?;

        let matches = self.fuzzy_sort(cleaned_input);
        Ok(matches.into_iter().map(|(path, _)| path).collect())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        // println!("TAB!");
        let cleaned_input = &*input.replace(self._opposite_slash, &*self._os_slash.to_string());
        self.update_input(cleaned_input)?;

        Ok(if let Some(suggestion) = highlighted_suggestion {
            Some(suggestion)
        } else {
            let matches = self.fuzzy_sort(cleaned_input);
            matches
                .first()
                .map(|(path, _)| Some(path.clone()))
                .unwrap_or(None)
        })
    }
}
