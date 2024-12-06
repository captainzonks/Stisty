use crate::data_types::csv::{import_csv_data, CSVData};
use anyhow::{Error, Result};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use inquire::{
    autocompletion::{Autocomplete, Replacement},
    CustomUserError, Select, Text,
};
use log::info;
use std::env;
use std::io::ErrorKind;
use std::path::Path;

pub fn menu() -> Result<(), Error> {
    let statistics = vec![
        "Single Sample T",
        "Paired Sample T",
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

    let csv_data: CSVData;

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

    let _statistic = Select::new("What statistic would you like to run?", statistics).prompt()?;

    info!("Statistic: {:?}", _statistic);

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

        let mut input_path = std::path::PathBuf::from(self.input.clone());

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
