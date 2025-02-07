#[cfg(test)]
mod tests {
    use anyhow::Error;
    // use assert_cmd::prelude::*; // Add methods on commands
    // use predicates::prelude::predicate;
    // use predicates::prelude::*; // Used for writing assertions
    use csv::ReaderBuilder;
    use rust_embed::Embed;
    // use std::process::Command;
    use stisty_lib;
    use stisty_lib::data_types::csv::CSVData;

    #[derive(Embed)]
    #[folder = "$CARGO_MANIFEST_DIR/tests/"]
    struct TestData;

    #[test]
    fn csv_import_and_column_extraction_is_okay() -> Result<(), Error> {
        let csv_test_data_option = TestData::get("test_data.csv");
        assert!(csv_test_data_option.is_some());
        let csv_test_data = csv_test_data_option.unwrap();
        // println!("{:#?}", std::str::from_utf8(csv_test_data.data.as_ref()));

        let mut reader = ReaderBuilder::new().from_reader(csv_test_data.data.as_ref());

        let mut column_count = 0;

        let mut sample_data: CSVData = Default::default();

        sample_data.headers = reader.headers()?.clone().iter().map(String::from).collect();

        for result in reader.records() {
            assert!(result.is_ok());
            let record = result?;
            assert!(record.iter().count() > 0);
            sample_data.total_columns = record.len();
            column_count += 1;
            for string in record.iter() {
                sample_data.data.push(string.to_string().trim().to_string());
                // trim in case of whitespace
            }
        }
        sample_data.total_rows = column_count;

        let extracted_string_column_result = &sample_data.get_column::<String>(1, None);
        assert!(extracted_string_column_result.is_ok());
        assert_eq!(
            *extracted_string_column_result.clone()?.get(0).unwrap(),
            String::from("Astronomy")
        );

        let extracted_numerical_column_result = &sample_data.get_column::<i32>(4, None);
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
