#[cfg(test)]
mod tests {
    use anyhow::Error;
    // use assert_cmd::prelude::*; // Add methods on commands
    // use predicates::prelude::predicate;
    // use predicates::prelude::*; // Used for writing assertions
    use std::path::Path;
    // use std::process::Command;
    use stisty_lib;
    // use stisty_lib::core::error_types::CSVError;
    use stisty_lib::data_types::csv::CSVData;

    #[test]
    fn csv_import_and_column_extraction_is_okay() -> Result<(), Error> {
        let test_data_path = Path::new("tests/test_data.csv");
        let csv_import_result =
            stisty_lib::data_types::csv::import_csv_data(test_data_path, None, None);

        let csv_import: CSVData;
        assert!(csv_import_result.is_ok());
        csv_import = csv_import_result?;

        let extracted_string_column_result = &csv_import.get_column::<String>(1, None);
        assert!(extracted_string_column_result.is_ok());
        assert_eq!(
            *extracted_string_column_result.clone()?.get(0).unwrap(),
            String::from("Astronomy")
        );

        let extracted_numerical_column_result = &csv_import.get_column::<i32>(4, None);
        assert!(extracted_numerical_column_result.is_ok());
        assert_eq!(
            *extracted_numerical_column_result.clone()?.get(0).unwrap(),
            12
        );

        Ok(())
    }

    // #[test]
    // fn file_doesnt_exist() -> Result<(), Error> {
    //     let mut cmd = Command::cargo_bin("stisty")?;
    //
    //     cmd.arg("-C").arg("-c").arg("./csv-files/test_data.csv");
    //     cmd.assert()
    //         .failure()
    //         .stderr(predicate::str::contains("could not read file"));
    //
    //     Ok(())
    // }
}
