use crate::core::arg_handler::{
    ANOVAConfig, DescriptionConfig, IndependentGroupsTConfig, PairedSamplesTConfig,
    SingleSampleTConfig,
};
use crate::core::logging;
use crate::data_types::data_array::{CategoricalDataArray, ContinuousDataArray};
use crate::functions::stats_math::{differences, mean, pooled_variance, variance};
use anyhow::{anyhow, Error, Result};
use log::info;

#[derive(Debug, Clone)]
pub struct SingleSampleT<'a> {
    pub name: String,
    pub description: String,
    _n: usize,
    _df: usize,

    _data: &'a ContinuousDataArray,

    // provided
    _mu: f64,

    // calculated
    _variance: f64,
    _standard_deviation: f64,

    _statistic_run: bool,
    pub t: f64,
}

impl<'a> SingleSampleT<'a> {
    pub fn new(
        name: String,
        description: String,
        data: &'a ContinuousDataArray,
        mu: f64,
    ) -> Result<SingleSampleT<'a>, Error> {
        let new_sst = SingleSampleT {
            name,
            description,
            _n: data.data_array.data.len(),
            _df: data.data_array.data.len() - 1,
            _data: data,
            _mu: mu,
            _variance: data.variance,
            _standard_deviation: data.standard_deviation,
            _statistic_run: false,
            t: 0.0,
        };

        // new_sst.run_statistic()?;

        Ok(new_sst)
    }

    fn run_statistic(&mut self) -> Result<(), Error> {
        // t = (x_bar - mu) / (sd / sqrt(n))

        info!("...Calculating 'Single Sample t'...");
        self._n = self._data.data_array.data.len();
        self._df = self._n - 1;
        self.t =
            (self._data.mean - self._mu) / (self._standard_deviation / f64::sqrt(self._n as f64));
        self._statistic_run = true;
        Ok(())
    }

    pub fn print(mut self) {
        if self._statistic_run {
            info!("Column: {}", self._data.column_header);
            info!("n: {}", self._n);
            info!("df: {}", self._df);
            info!("mean: {}", self._data.mean);
            info!("mu: {}", self._mu);
            info!("standard deviation: {}", self._data.standard_deviation);
            info!("variance: {}", self._data.variance);
            info!("Single Sample t = {}", self.t)
        } else {
            self.run_statistic()
                .expect("Error running single sample t test");
            self.print();
        }
    }
}

pub fn run_single_sample_t_test(config: SingleSampleTConfig) -> Result<(), Error> {
    let mut description_config_in: DescriptionConfig = Default::default();
    if let Some(description_config) = config.description_config {
        description_config_in = description_config;
    } else {
        description_config_in.name = String::from("Single Sample t Test");
        description_config_in.description = String::from("Single Sample t Test");
    }

    let new_data_array: ContinuousDataArray = ContinuousDataArray::new(
        description_config_in.name.clone(),
        &config
            .csv_data
            .get_column::<f64>(config.column_index, Some(false))?,
        config.column_index,
        config.csv_data.headers[config.column_index].clone(),
        Some(false),
    )?;

    let mut new_single_sample_t = SingleSampleT::new(
        description_config_in.name,
        description_config_in.description,
        &new_data_array,
        config.mu,
    )?;
    new_single_sample_t.run_statistic()?;
    new_single_sample_t.print();

    Ok(())
}

#[derive(Debug, Clone)]
pub struct PairedSamplesT<'a> {
    pub name: String,
    pub description: String,
    _n: usize,
    _df: usize,

    // provided
    _data_x: &'a ContinuousDataArray,
    _data_y: &'a ContinuousDataArray,

    // calculated
    _differences: Vec<f64>,
    _mean_of_differences: f64,
    _sum_of_squares_differences: f64,
    _variance_of_differences: f64,
    _s_sub_d_bar: f64,

    _statistic_run: bool,
    pub t: f64,
}

impl<'a> PairedSamplesT<'a> {
    pub fn new(
        name: String,
        description: String,
        data_x: &'a ContinuousDataArray,
        data_y: &'a ContinuousDataArray,
    ) -> Result<PairedSamplesT<'a>, Error> {
        if data_x.data_array.data.len() == data_y.data_array.data.len() {
            let new_pst = PairedSamplesT {
                name,
                description,
                _n: data_x.data_array.data.len(),
                _df: data_x.data_array.data.len() - 1,
                _data_x: data_x,
                _data_y: data_y,
                _differences: vec![],
                _mean_of_differences: 0.0,
                _sum_of_squares_differences: 0.0,
                _variance_of_differences: 0.0,
                _s_sub_d_bar: 0.0,
                _statistic_run: false,
                t: 0.0,
            };

            // new_pst.run_statistic()?;

            Ok(new_pst)
        } else {
            Err(anyhow!("provided data are not of same length"))
        }
    }

    fn run_statistic(&mut self) -> Result<(), Error> {
        if self._data_x.data_array.data.len() == self._data_y.data_array.data.len() {
            info!("...Calculating 'Paired Sample t'...");

            self._n = self._data_x.data_array.data.len();
            self._df = self._n - 1;

            let data_x = &self
                ._data_x
                .data_array
                .data
                .iter()
                .map(|x| x.1)
                .collect::<Vec<f64>>();
            let data_y = &self
                ._data_y
                .data_array
                .data
                .iter()
                .map(|y| y.1)
                .collect::<Vec<f64>>();
            self._differences = differences(data_x, data_y)?;
            self._mean_of_differences = self._differences.iter().sum::<f64>() / data_x.len() as f64;
            self._sum_of_squares_differences = self
                ._differences
                .iter()
                .map(|x| f64::powi(*x - self._mean_of_differences, 2))
                .sum::<f64>();
            self._variance_of_differences = self._sum_of_squares_differences / self._df as f64;
            self._s_sub_d_bar =
                f64::sqrt(self._variance_of_differences) / f64::sqrt(self._n as f64);
            self.t = (self._mean_of_differences - 0.0) / self._s_sub_d_bar;

            self._statistic_run = true;

            Ok(())
        } else {
            Err(anyhow!(
                "Data X and Data Y differ in lengths--cannot run 'Paired Sample t'"
            ))
        }
    }

    pub fn print(mut self) {
        if self._statistic_run {
            info!("df = {}", self._df);
            info!("Mean of Diff = {}", self._mean_of_differences);
            info!("Paired Sample t = {}", self.t)
        } else {
            self.run_statistic()
                .expect("Error running paired sample t test");
            self.print();
        }
    }
}

pub fn run_paired_samples_t_test(config: PairedSamplesTConfig) -> Result<(), Error> {
    let mut description_config_in: DescriptionConfig = Default::default();
    if let Some(description_config) = config.description_config {
        description_config_in = description_config;
    } else {
        description_config_in.name = String::from("Paired Samples t Test");
        description_config_in.description = String::from("Paired Samples t Test");
    }

    let new_data_array_x: ContinuousDataArray = ContinuousDataArray::new(
        description_config_in.name.clone(),
        &config
            .csv_data
            .get_column::<f64>(config.column_indices[0], Some(false))?,
        config.column_indices[0],
        config.csv_data.headers[config.column_indices[0]].clone(),
        Some(false),
    )?;

    let new_data_array_y: ContinuousDataArray = ContinuousDataArray::new(
        description_config_in.name.clone(),
        &config
            .csv_data
            .get_column::<f64>(config.column_indices[1], Some(false))?,
        config.column_indices[1],
        config.csv_data.headers[config.column_indices[1]].clone(),
        Some(false),
    )?;

    let mut new_paired_samples_t_test = PairedSamplesT::new(
        description_config_in.name,
        description_config_in.description,
        &new_data_array_x,
        &new_data_array_y,
    )?;
    new_paired_samples_t_test.run_statistic()?;
    new_paired_samples_t_test.print();

    Ok(())
}

#[derive(Debug, Clone)]
pub struct IndependentGroupsT<'a> {
    pub name: String,
    pub description: String,
    _level_row_indices: Vec<&'a Vec<usize>>,
    _level_names: Vec<String>,
    _df: usize,

    // provided
    _categorical_data: &'a CategoricalDataArray<'a>,
    _continuous_data: &'a ContinuousDataArray,

    // calculated
    _variance_level_1: f64,
    _variance_level_2: f64,
    _pooled_variance: f64,
    _standard_deviation_differences_between_means: f64,

    _statistic_run: bool,
    pub t: f64,
}

impl<'a> IndependentGroupsT<'a> {
    pub fn new(
        name: String,
        description: String,
        categorical_data: &'a CategoricalDataArray,
        continuous_data: &'a ContinuousDataArray,
    ) -> Result<IndependentGroupsT<'a>, Error> {
        if categorical_data.levels.keys().len() == 2 {
            let new_igt = IndependentGroupsT {
                name,
                description,
                _level_row_indices: Vec::with_capacity(
                    Vec::<usize>::with_capacity(categorical_data.levels.len()).len(),
                ),
                _level_names: Vec::with_capacity(categorical_data.levels.len()),
                _df: 0,
                _categorical_data: categorical_data,
                _continuous_data: continuous_data,
                _variance_level_1: 0.0,
                _variance_level_2: 0.0,
                _pooled_variance: 0.0,
                _standard_deviation_differences_between_means: 0.0,
                _statistic_run: false,
                t: 0.0,
            };

            // new_igt.run_statistic()?;

            Ok(new_igt)
        } else {
            Err(anyhow!(
                "A categorical variable with two levels is required to run an independent groups \
                t test"
            ))
        }
    }

    fn run_statistic(&mut self) -> Result<(), Error> {
        self._level_row_indices = self
            ._categorical_data
            .levels
            .iter()
            .map(|x| x.1)
            .collect::<Vec<&'a Vec<usize>>>();

        self._df = if self._categorical_data.n >= 2 {
            self._categorical_data.n - 2
        } else {
            0
        };

        let mut separated_continuous_data: Vec<Vec<&f64>> =
            Vec::with_capacity(self._continuous_data.n);

        for (level_name, _) in &self._categorical_data.levels {
            self._level_names.push(level_name.to_string());
            separated_continuous_data.push(
                self._categorical_data
                    .get_level_associated_continuous_data(level_name, &self._continuous_data)?,
            );
        }

        let level_1_continuous_data = &separated_continuous_data[0];
        let level_2_continuous_data = &separated_continuous_data[1];

        self._variance_level_1 =
            variance(level_1_continuous_data, self._continuous_data.population)?;
        self._variance_level_2 =
            variance(level_2_continuous_data, self._continuous_data.population)?;

        self._pooled_variance = pooled_variance(
            level_1_continuous_data,
            level_2_continuous_data,
            Some(&self._variance_level_1),
            Some(&self._variance_level_2),
        )?;

        self._standard_deviation_differences_between_means = f64::sqrt(
            (self._pooled_variance / self._level_row_indices[0].len() as f64)
                + (self._pooled_variance / self._level_row_indices[1].len() as f64),
        );

        self.t = (mean(&level_1_continuous_data)? - mean(&level_2_continuous_data)?)
            / self._standard_deviation_differences_between_means;

        self._statistic_run = true;

        Ok(())
    }

    pub fn print(mut self) {
        if self._statistic_run {
            info!("{}", logging::format_title(&*self.name));
            info!("Description: '{}'", self.description);
            info!("Level 1: '{}'", self._level_names[0]);
            info!("Level 2: '{}'", self._level_names[1]);
            info!("Variance Level 1: {}", self._variance_level_1);
            info!("Variance Level 2: {}", self._variance_level_2);
            info!("Pooled variance: {}", self._pooled_variance);
            info!(
                "Standard Deviation: {}",
                self._standard_deviation_differences_between_means
            );
            info!("Independent Groups t: {}", self.t);
        } else {
            self.run_statistic().expect("Error running statistic");
            self.print();
        }
    }
}

pub fn run_independent_groups_t_test(config: IndependentGroupsTConfig) -> Result<(), Error> {
    let mut description_config_in: DescriptionConfig = Default::default();
    if let Some(description_config) = config.description_config {
        description_config_in = description_config;
    } else {
        description_config_in.name = String::from("Independent Groups t Test");
        description_config_in.description = String::from("Independent Groups t Test");
    }

    let categorical_data_column = config
        .csv_data
        .get_column::<String>(config.categorical_column_index, Some(false))?;

    let categorical_data_array: CategoricalDataArray = CategoricalDataArray::new(
        description_config_in.name.clone(),
        &categorical_data_column,
        config.categorical_column_index,
        config.csv_data.headers[config.categorical_column_index].clone(),
        Some(false),
    )?;

    let continuous_data_array: ContinuousDataArray = ContinuousDataArray::new(
        description_config_in.name.clone(),
        &config
            .csv_data
            .get_column::<f64>(config.continuous_column_index, Some(false))?,
        config.continuous_column_index,
        config.csv_data.headers[config.continuous_column_index].clone(),
        Some(false),
    )?;

    let mut new_independent_groups_t_test = IndependentGroupsT::new(
        description_config_in.name,
        description_config_in.description,
        &categorical_data_array,
        &continuous_data_array,
    )?;
    new_independent_groups_t_test.run_statistic()?;
    new_independent_groups_t_test.print();

    Ok(())
}

#[derive(Debug, Clone)]
pub struct ZTest<'a> {
    pub name: String,
    pub description: String,
    pub n: usize,
    pub df: usize,

    pub data: &'a ContinuousDataArray,

    // provided
    pub mu: f64,
    pub standard_deviation: f64,

    // calculated
    pub z: f64,
}

pub struct ANOVA<'a> {
    pub name: String,
    pub description: String,
    _one_way: bool,
    _level_row_indices: Vec<&'a Vec<usize>>,
    _df_b: usize,
    _df_w: usize,

    // provided
    _categorical_data: &'a CategoricalDataArray<'a>,
    _continuous_data: &'a ContinuousDataArray,

    // calculated
    _level_means: Vec<f64>,
    _grand_mean: f64,

    _sum_of_squares_between_groups: f64,
    _sum_of_squares_within_groups: f64,

    _mean_square_between_groups: f64,
    _mean_square_within_groups: f64,

    _statistic_run: bool,
    pub f: f64,
}

impl<'a> ANOVA<'a> {
    pub fn new(
        name: String,
        description: String,
        categorical_data: &'a CategoricalDataArray,
        continuous_data: &'a ContinuousDataArray,
        one_way: Option<bool>,
    ) -> Result<ANOVA<'a>, Error> {
        if categorical_data.levels.len() >= 3 {
            let new_anova = ANOVA {
                name,
                description,
                _one_way: one_way.unwrap_or(true),
                _level_row_indices: Vec::with_capacity(
                    Vec::<usize>::with_capacity(categorical_data.levels.len()).len(),
                ),
                _df_b: categorical_data.levels.len() - 1,
                _df_w: 0,
                _categorical_data: categorical_data,
                _continuous_data: continuous_data,
                _level_means: Vec::with_capacity(categorical_data.levels.len()),
                _grand_mean: 0.0,
                _sum_of_squares_between_groups: 0.0,
                _sum_of_squares_within_groups: 0.0,
                _mean_square_between_groups: 0.0,
                _mean_square_within_groups: 0.0,
                _statistic_run: false,
                f: 0.0,
            };

            // new_anova.run_statistic()?;

            Ok(new_anova)
        } else {
            Err(anyhow!(
                "Categorical data consisting of at least three levels is required for a \
            one way ANOVA test"
            ))
        }
    }

    fn run_statistic(&mut self) -> Result<(), Error> {
        self._level_row_indices = self
            ._categorical_data
            .levels
            .iter()
            .map(|x| x.1)
            .collect::<Vec<&'a Vec<usize>>>();

        self._df_w = self._continuous_data.n - self._categorical_data.levels.len();

        let mut separated_continuous_data: Vec<Vec<&f64>> =
            Vec::with_capacity(self._continuous_data.n);

        for (level_name, _) in self._categorical_data.levels.iter() {
            separated_continuous_data.push(
                self._categorical_data
                    .get_level_associated_continuous_data(level_name, self._continuous_data)?,
            );
        }

        for i in 0..separated_continuous_data.len() {
            self._level_means.push(mean(&separated_continuous_data[i])?);
        }

        self._grand_mean = self
            ._level_means
            .iter()
            .enumerate()
            .map(|(index, mean)| mean * self._level_row_indices[index].len() as f64)
            .sum::<f64>()
            / self._continuous_data.data_array.data.len() as f64;

        self._sum_of_squares_between_groups = self
            ._level_means
            .iter()
            .enumerate()
            .map(|(index, mean)| {
                f64::powi(mean - self._grand_mean, 2) * self._level_row_indices[index].len() as f64
            })
            .sum::<f64>();
        self._mean_square_between_groups = self._sum_of_squares_between_groups / self._df_b as f64;

        self._sum_of_squares_within_groups = separated_continuous_data
            .iter()
            .enumerate()
            .map(|(index, data_set)| {
                data_set
                    .iter()
                    .map(|datum| f64::powi(*datum - self._level_means[index], 2))
                    .sum::<f64>()
            })
            .sum::<f64>();

        self._mean_square_within_groups = self._sum_of_squares_within_groups / self._df_w as f64;

        self.f = self._mean_square_between_groups / self._mean_square_within_groups;

        self._statistic_run = true;

        Ok(())
    }

    pub fn print(mut self) {
        if self._statistic_run {
            info!("{}", logging::format_title(&*self.name));
            info!("Description: '{}'", self.description);
            for (index, (level_name, _)) in self._categorical_data.levels.iter().enumerate() {
                info!("Level {}: {}", index, level_name);
                info!("..n: {}", self._level_row_indices[index].len());
                info!("..mean: {}", self._level_means[index]);
            }
            info!("Grand Mean: {}", self._grand_mean);
            info!("dfB: {}", self._df_b);
            info!("dfW: {}", self._df_w);
            info!("SSB: {}", self._sum_of_squares_between_groups);
            info!("SSW: {}", self._sum_of_squares_within_groups);
            info!("MSB: {}", self._mean_square_between_groups);
            info!("MSW: {}", self._mean_square_within_groups);
            info!("F: {}", self.f);
        } else {
            self.run_statistic().expect("Error running statistic");
            self.print();
        }
    }
}

pub fn run_anova_test(config: ANOVAConfig) -> Result<(), Error> {
    let mut description_config_in: DescriptionConfig = Default::default();
    if let Some(description_config) = config.description_config {
        description_config_in = description_config;
    } else {
        description_config_in.name = String::from("ANOVA Test");
        description_config_in.description = String::from("ANOVA Test");
    }

    let categorical_data_column = config
        .csv_data
        .get_column::<String>(config.categorical_column_index, Some(false))?;

    let categorical_data_array: CategoricalDataArray = CategoricalDataArray::new(
        description_config_in.name.clone(),
        &categorical_data_column,
        config.categorical_column_index,
        config.csv_data.headers[config.categorical_column_index].clone(),
        Some(false),
    )?;

    let continuous_data_array: ContinuousDataArray = ContinuousDataArray::new(
        description_config_in.name.clone(),
        &config
            .csv_data
            .get_column::<f64>(config.continuous_column_index, Some(false))?,
        config.continuous_column_index,
        config.csv_data.headers[config.continuous_column_index].clone(),
        Some(false),
    )?;

    let mut new_anova_test = ANOVA::new(
        description_config_in.name,
        description_config_in.description,
        &categorical_data_array,
        &continuous_data_array,
        Some(true),
    )?;
    new_anova_test.run_statistic()?;
    new_anova_test.print();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{IndependentGroupsT, PairedSamplesT, SingleSampleT, ANOVA};
    // use crate::data_types::csv::generate_dummy_csv;
    use crate::data_types::data_array::{CategoricalDataArray, ContinuousDataArray};
    use anyhow::{Error, Result};

    #[test]
    fn single_sample_t_is_ok() -> Result<(), Error> {
        // let dummy_data = generate_dummy_csv();
        let continuous_data_array = ContinuousDataArray::new(
            String::from("Continuous Data Array"),
            &vec![22.0, 13.2, 7.9, 0.1, 9.34, 42.722],
            0,
            String::from("Dummy test data"),
            None,
        )?;
        let mut test = SingleSampleT::new(
            String::from("Single Sample t Test"),
            String::from("Single Sample t Test"),
            &continuous_data_array,
            11.6,
        )?;
        test.run_statistic()?;

        // round the float off to test against value obtained from R function
        assert_eq!(f64::round(test.t * 10000000.0) / 10000000.0, 0.6998041);
        Ok(())
    }

    #[test]
    fn paired_samples_t_is_ok() -> Result<(), Error> {
        let continuous_data_array_01 = ContinuousDataArray::new(
            String::from("Continuous Data Array"),
            &vec![51.2, 63.44, 98.10, 10.5, 36.7, 92.32],
            0,
            String::from("Dummy continuous test data 1"),
            None,
        )?;

        let continuous_data_array_02 = ContinuousDataArray::new(
            String::from("Continuous Data Array"),
            &vec![152.2, 124.9, 177.0, 110.4, 75.2, 189.77],
            1,
            String::from("Dummy continuous test data 2"),
            None,
        )?;

        let mut test = PairedSamplesT::new(
            String::from("Paired Samples t Test"),
            String::from("Paired Samples t Test"),
            &continuous_data_array_01,
            &continuous_data_array_02,
        )?;
        test.run_statistic()?;

        // round the float off to test against value obtained from R function
        assert_eq!(f64::round(test.t * 1000000.0) / 1000000.0, 7.692046);

        Ok(())
    }

    #[test]
    fn independent_groups_t_is_ok() -> Result<(), Error> {
        // let dummy_data = generate_dummy_csv();
        let categorical_strings = String::from("Yes Yes Yes Yes No No No Yes")
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let categorical_data_array = CategoricalDataArray::new(
            String::from("Categorical Data Array"),
            &categorical_strings,
            0,
            String::from("Dummy categorical test data"),
            None,
        )?;

        let continuous_data_array = ContinuousDataArray::new(
            String::from("Continuous Data Array"),
            &vec![12.0, 56.2, 32.9, 9.34, 41.21, 16.01, 2.0, 8.999],
            1,
            String::from("Dummy continuous test data"),
            None,
        )?;

        let mut test = IndependentGroupsT::new(
            String::from("Independent Groups t Test"),
            String::from("Independent Groups t Test"),
            &categorical_data_array,
            &continuous_data_array,
        )?;
        test.run_statistic()?;

        // round the float off to test against value obtained from R function
        assert_eq!(f64::round(f64::abs(test.t) * 10000000.0) / 10000000.0, 0.2788283);

        Ok(())
    }

    #[test]
    fn one_way_anova_is_ok() -> Result<(), Error> {
        let categorical_strings = String::from("Red Blue Blue Red Green Blue Red Blue")
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let categorical_data_array = CategoricalDataArray::new(
            String::from("Categorical Data Array"),
            &categorical_strings,
            0,
            String::from("Dummy categorical test data"),
            None,
        )?;

        let continuous_data_array = ContinuousDataArray::new(
            String::from("Continuous Data Array"),
            &vec![77.2, 89.3, 12.1, 99.9, 2.4, 15.6, 25.66, 98.101],
            1,
            String::from("Dummy continuous test data"),
            None,
        )?;

        let mut test = ANOVA::new(
            String::from("One Way ANOVA Test"),
            String::from("One Way ANOVA Test"),
            &categorical_data_array,
            &continuous_data_array,
            Some(true),
        )?;
        test.run_statistic()?;

        assert_eq!(f64::round(test.f * 1000.0) / 1000.0, 0.859);

        Ok(())
    }
}

//         // y-hat = beta(x) + alpha
//         // x = (y-hat - alpha) / beta
//         // beta = (y-hat - alpha) / x
//         // alpha = y-hat - (beta)x
//
//         new_relationship.slope_beta =
//             new_relationship.covariance / new_relationship.data_x.variance;
//
//         new_relationship.intercept_alpha = new_relationship.data_y.mean
//             - new_relationship.slope_beta * new_relationship.data_x.mean;
//
//         // beta_hat = sum((x_i - x-bar)(y - y-bar)) / sum((x_i - x-bar)^2)
//         new_relationship.slope_beta_hat =
//             new_relationship.sum_of_product_of_deviations / new_relationship.data_x.sum_of_squares;
//
//         // alpha_hat = y-bar - (beta_hat * x-bar)
//         new_relationship.intercept_alpha_hat = new_relationship.data_y.mean
//             - (new_relationship.slope_beta_hat * new_relationship.data_x.mean);
//
//         // calculate fitted
//         for datum in new_relationship.data_x.data.iter() {
//             new_relationship
//                 .fitted_values
//                 .push(new_relationship.get_y_hat(*datum));
//         }
//
//         // calculate and collect residuals
//         let mut fitted_iter = new_relationship.fitted_values.iter();
//         new_relationship.residuals = new_relationship
//             .data_y
//             .data
//             .iter()
//             .map(|y_i| y_i - fitted_iter.next().unwrap())
//             .collect();

//         // ESS = sum((fitted_y - y_mean)^2)
//         new_relationship.explained_sum_of_squares = new_relationship
//             .fitted_values
//             .iter()
//             .map(|fitted_y| f64::powi(fitted_y - new_relationship.data_y.mean, 2))
//             .sum::<f64>();

//         // SSE = sum(residual^2)
//         new_relationship.sum_of_squares_error = new_relationship
//             .residuals
//             .iter()
//             .map(|residual| f64::powi(*residual, 2))
//             .sum::<f64>();

//         // SST (treatment sum of squares)
//          SST = sum( for every group ( n.i * x.i.mean ^ 2 - n.all * x.grand_mean ^ 2 )

//         // MSR = ESS / p, or MSR = ESS (since p = 1)
//         new_relationship.mean_square_regression =
//             new_relationship.explained_sum_of_squares / new_relationship.p;

//         // MSE = SSE / n
//         new_relationship.mean_square_error = new_relationship.sum_of_squares_error
//             / (new_relationship.n_all - new_relationship.p - 1.0);

//         // F-statistic, one-way ANOVA Type 1
//         new_relationship.one_way_anova_f_statistic =
//             new_relationship.mean_square_regression / new_relationship.mean_square_error;

//
// #[derive(Debug, Clone)]
// pub struct DataRelationship<'a> {
//     pub name: String,
//     pub n_all: f64,
//     pub p: f64, // total regressors
//
//     pub continuous_data: Vec<&'a ContinuousDataArray>,
//     pub categorical_data: Option<&'a CategoricalDataArray<'a>>,
//
//     pub differences: Vec<f64>,
//     pub sum_of_differences: f64,
//     pub mean_of_differences: f64,
//     pub sum_of_squares_differences: f64,
//     pub variance_of_differences: f64,
//     pub s_sub_d_bar: f64,
//
//     pub sum_of_product_of_z_scores: f64,
//     pub sum_of_product_of_deviations: f64,
//
//     pub pooled_variance: f64,
//     pub standard_deviation_differences_between_means: f64,
//     pub independent_groups_t: f64,
//
//     pub covariance: f64,
//     pub pearson_r: f64,
//
//     pub slope_beta: f64,          // biased
//     pub slope_beta_hat: f64,      // unbiased
//     pub intercept_alpha: f64,     // biased
//     pub intercept_alpha_hat: f64, // unbiased
//     pub t_score_coefficient: f64, // (from Pearson r) r * sqrt(N - 2) / sqrt(1 - r^2)
//     pub t_score_intercept: f64,   // intercept / standard error of intercept
//     pub paired_sample_t_test: f64,
//
//     pub standard_error_of_regression_slope: f64, // SE(Beta-hat) = sqrt((1/(n-p-1)*MSE)/SSx)
//     pub standard_error_of_regression_intercept: f64, // SE(alpha-hat) = SE(Beta-hat) * sqrt((1/n)*sum(x^2))
//
//     pub fitted_values: Vec<f64>, // y-hat
//     pub residuals: Vec<f64>,     // e_i = y_i - y-hat
//
//     pub sum_of_squares_total: f64,     // SST
//     pub sum_of_squares_error: f64,     // SSE
//     pub explained_sum_of_squares: f64, // ESS
//
//     pub mean_square_regression: f64, // MSR = ESS / p, or MSR = ESS in simple linear regression
//     pub mean_square_error: f64,      // MSE = SSE / (n - p); standard error of the estimate
//     pub residual_standard_error: f64, // sqrt((1/(n - p - 1)) * SSE)
//
//     // proportion of observed y variation that can be explained by the simple linear regression model
//     pub coefficient_of_determination: f64,          // R^2
//     pub coefficient_of_determination_adjusted: f64, // R^2 adjusted
//
//     pub one_way_anova_f_statistic: f64, // Type 1
// }
//
// impl<'a> DataRelationship<'a> {
//     pub fn new(
//         name: String,
//         continuous_data: Option<Vec<&ContinuousDataArray>>,
//         categorical_data: Option<Vec<&CategoricalDataArray<'a>>>,
//     ) -> Result<DataRelationship<'a>, Error> {
//         if continuous_data.is_none() || categorical_data.is_none() {
//             return Err(anyhow!("No data provided from which to build relationship"));
//         }
//
//         let mut new_relationship: DataRelationship = DataRelationship::default();
//
//         new_relationship.name = name;
//
//         match continuous_data {
//             None => {}
//             Some(data) => new_relationship.continuous_data = data,
//         }
//         match categorical_data {
//             None => {}
//             Some(data) => new_relationship.categorical_data = data,
//         }
//
//         new_relationship.n_all = new_relationship;
//
//         new_relationship.p = 1.0; // simple linear regression has only one regressor
//
//         new_relationship.data_x = data_x.clone();
//         new_relationship.data_y = data_y.clone();
//
//         // Differences of Data
//         let mut iter = new_relationship.data_x.data.iter();
//         new_relationship.differences = new_relationship
//             .data_y
//             .data
//             .iter()
//             .map(|x| x - iter.next().unwrap())
//             .collect();
//
//         // Sum of Differences
//         new_relationship.sum_of_differences = new_relationship.differences.iter().sum::<f64>();
//
//         // Mean of the Differences
//         new_relationship.mean_of_differences =
//             new_relationship.sum_of_differences / new_relationship.n_all;
//
//         // Sum of Squares of Differences
//         new_relationship.sum_of_squares_differences = new_relationship
//             .differences
//             .iter()
//             .map(|x| f64::powi(*x - new_relationship.mean_of_differences, 2))
//             .sum::<f64>();
//
//         // Variance of Differences
//         new_relationship.variance_of_differences = new_relationship.sum_of_squares_differences
//             / (new_relationship.n_all
//                 - if new_relationship.data_x.population.unwrap_or_default() {
//                     0.0
//                 } else {
//                     1.0
//                 });
//
//         // Standard Deviation of the Differences
//         new_relationship.s_sub_d_bar = f64::sqrt(new_relationship.variance_of_differences);
//
//         // Paired Sample t Test = (mean(differences) - 0) / (standard deviation of the differences / sqrt(n))
//         new_relationship.paired_sample_t_test = (new_relationship.mean_of_differences - 0.0)
//             / (new_relationship.s_sub_d_bar / f64::sqrt(new_relationship.n_all));
//
//         // Sum of Product of Z-Scores
//         iter = new_relationship.data_y.z_scores.iter();
//         new_relationship.sum_of_product_of_z_scores = new_relationship
//             .data_x
//             .z_scores
//             .iter()
//             .map(|x_z| x_z * iter.next().unwrap())
//             .sum::<f64>();
//
//         // Sum of Product of Deviations
//         iter = new_relationship.data_y.deviations.iter();
//         new_relationship.sum_of_product_of_deviations = new_relationship
//             .data_x
//             .deviations
//             .iter()
//             .map(|dev_x| dev_x * iter.next().unwrap())
//             .sum::<f64>();
//
//         // Covariance = (sum(data_x's deviations * data_y's deviations)) / (N (- 1, for Bessel's Correction))
//         new_relationship.covariance = new_relationship.sum_of_product_of_deviations
//             / (new_relationship.n_all
//                 - if new_relationship.data_x.population.unwrap_or_default() {
//                     0.0
//                 } else {
//                     1.0
//                 });
//
//         // Pearson r = covariance / (data_x's sd * data_y's sd)
//         new_relationship.pearson_r = new_relationship.covariance
//             / (new_relationship.data_x.standard_deviation
//                 * new_relationship.data_y.standard_deviation);
//
//         // t-score for coefficient (from Pearson r) = r * sqrt(N - 2) / sqrt(1 - r^2)
//         new_relationship.t_score_coefficient = new_relationship.pearson_r
//             * f64::sqrt(new_relationship.n_all - new_relationship.p - 1.0)
//             / f64::sqrt(1.0 - f64::powi(new_relationship.pearson_r, 2));
//
//         // y-hat = beta(x) + alpha
//         // x = (y-hat - alpha) / beta
//         // beta = (y-hat - alpha) / x
//         // alpha = y-hat - (beta)x
//
//         new_relationship.slope_beta =
//             new_relationship.covariance / new_relationship.data_x.variance;
//
//         new_relationship.intercept_alpha = new_relationship.data_y.mean
//             - new_relationship.slope_beta * new_relationship.data_x.mean;
//
//         // beta_hat = sum((x_i - x-bar)(y - y-bar)) / sum((x_i - x-bar)^2)
//         new_relationship.slope_beta_hat =
//             new_relationship.sum_of_product_of_deviations / new_relationship.data_x.sum_of_squares;
//
//         // alpha_hat = y-bar - (beta_hat * x-bar)
//         new_relationship.intercept_alpha_hat = new_relationship.data_y.mean
//             - (new_relationship.slope_beta_hat * new_relationship.data_x.mean);
//
//         // calculate fitted
//         for datum in new_relationship.data_x.data.iter() {
//             new_relationship
//                 .fitted_values
//                 .push(new_relationship.get_y_hat(*datum));
//         }
//
//         // calculate and collect residuals
//         let mut fitted_iter = new_relationship.fitted_values.iter();
//         new_relationship.residuals = new_relationship
//             .data_y
//             .data
//             .iter()
//             .map(|y_i| y_i - fitted_iter.next().unwrap())
//             .collect();
//
//         // SST = sum((y_i - y_mean)^2)
//         new_relationship.sum_of_squares_total = new_relationship
//             .data_y
//             .data
//             .iter()
//             .map(|y_i| f64::powi(y_i - new_relationship.data_y.mean, 2))
//             .sum::<f64>();
//
//         // SSE = sum(residual^2)
//         new_relationship.sum_of_squares_error = new_relationship
//             .residuals
//             .iter()
//             .map(|residual| f64::powi(*residual, 2))
//             .sum::<f64>();
//
//         // ESS = sum((fitted_y - y_mean)^2)
//         new_relationship.explained_sum_of_squares = new_relationship
//             .fitted_values
//             .iter()
//             .map(|fitted_y| f64::powi(fitted_y - new_relationship.data_y.mean, 2))
//             .sum::<f64>();
//
//         // MSR = ESS / p, or MSR = ESS (since p = 1)
//         new_relationship.mean_square_regression =
//             new_relationship.explained_sum_of_squares / new_relationship.p;
//
//         // MSE = SSE / n
//         new_relationship.mean_square_error = new_relationship.sum_of_squares_error
//             / (new_relationship.n_all - new_relationship.p - 1.0);
//
//         // RSE = sqrt((1/(n - p - 1)) * SSE)
//         new_relationship.residual_standard_error = f64::sqrt(
//             (1.0 / (new_relationship.n_all - new_relationship.p - 1.0))
//                 * new_relationship.sum_of_squares_error,
//         );
//
//         // SE(Beta-hat) = sqrt((1/(n-p-1))*(MSE/SSx))
//         new_relationship.standard_error_of_regression_slope = f64::sqrt(
//             (1.0 / (new_relationship.n_all - new_relationship.p - 1.0))
//                 * (new_relationship.sum_of_squares_error / new_relationship.data_x.sum_of_squares),
//         );
//
//         // SE(alpha-hat) = SE(Beta-hat) * sqrt((1/n)*sum(x^2))
//         new_relationship.standard_error_of_regression_intercept = new_relationship
//             .standard_error_of_regression_slope
//             * f64::sqrt(
//                 (1.0 / new_relationship.n_all)
//                     * new_relationship
//                         .data_x
//                         .data
//                         .iter()
//                         .map(|x| f64::powi(*x, 2))
//                         .collect::<Vec<f64>>()
//                         .iter()
//                         .sum::<f64>(),
//             );
//
//         // t-score for the intercept (alpha / standard error of intercept)
//         new_relationship.t_score_intercept = new_relationship.intercept_alpha
//             / new_relationship.standard_error_of_regression_intercept;
//
//         // ESS, cheaper method (and perhaps not completely accurate)
//         // new_relationship.explained_sum_of_squares = new_relationship.sum_of_squares_total
//         // - new_relationship.sum_of_squares_error;
//
//         // coefficient of multiple determination, or R^2 = ESS/SST
//         new_relationship.coefficient_of_determination =
//             new_relationship.explained_sum_of_squares / new_relationship.sum_of_squares_total;
//
//         // R^2 adjusted = 1 - (1 - R^2) * ((n - 1) / (n - p - 1))
//         new_relationship.coefficient_of_determination_adjusted = 1.0
//             - ((1.0 - new_relationship.coefficient_of_determination)
//                 * ((new_relationship.n_all - 1.0)
//                     / (new_relationship.n_all - new_relationship.p - 1.0)));
//
//         // F-statistic, one-way ANOVA Type 1
//         new_relationship.one_way_anova_f_statistic =
//             new_relationship.mean_square_regression / new_relationship.mean_square_error;
//
//         // Pooled variance for independent groups t test
//         new_relationship.pooled_variance = ((new_relationship.data_x.data.len() as f64 - 1.0)
//             * new_relationship.data_x.variance
//             + (new_relationship.data_y.data.len() as f64 - 1.0) * new_relationship.data_y.variance)
//             / (new_relationship.data_x.data.len() as f64
//                 + new_relationship.data_y.data.len() as f64
//                 - 2.0);
//
//         // Standard deviation of the differences between the means
//         new_relationship.standard_deviation_differences_between_means =
//             ((new_relationship.pooled_variance / new_relationship.data_x.data.len() as f64)
//                 + (new_relationship.pooled_variance / new_relationship.data_y.data.len() as f64))
//                 .sqrt();
//
//         // Independent groups t
//         new_relationship.independent_groups_t = (new_relationship.data_x.mean
//             - new_relationship.data_y.mean)
//             / new_relationship.standard_deviation_differences_between_means;
//
//         Ok(new_relationship)
//     }
//
//     pub fn get_y_hat(&self, x_value: f64) -> f64 {
//         self.slope_beta * x_value + self.intercept_alpha
//     }
//
//     pub fn get_x(&self, y_value: f64) -> f64 {
//         (y_value - self.intercept_alpha) / self.slope_beta
//     }
//
//     pub fn get_intercept_alpha(&self, y_value: f64, x_value: f64) -> f64 {
//         y_value - self.intercept_alpha * x_value
//     }
//
//     pub fn get_slope_beta(&self, y_value: f64, x_value: f64) -> f64 {
//         (y_value - self.intercept_alpha) / x_value
//     }
//
//     pub fn print_relationship(&self) {
//         info!("{}", logging::format_title(&*self.name));
//         info!("n................................{}", self.n_all);
//         info!("p................................{}", self.p);
//         info!(
//             "df...............................{}",
//             self.n_all - self.p - 1.0
//         );
//         info!("Data X mean......................{}", self.data_x.mean);
//         info!("Data Y mean......................{}", self.data_y.mean);
//         info!(
//             "Sum of Product of Z-Scores.......{}",
//             self.sum_of_product_of_z_scores
//         );
//         info!(
//             "Sum of Product of Deviations.....{}",
//             self.sum_of_product_of_deviations
//         );
//         // info!("Differences......................{:?}", self.differences);
//         info!(
//             "Sum of Differences...............{}",
//             self.sum_of_differences
//         );
//         info!(
//             "Mean of Differences..............{}",
//             self.mean_of_differences
//         );
//         info!(
//             "Variance of Differences..........{}",
//             self.variance_of_differences
//         );
//         info!("S sub D-bar......................{}", self.s_sub_d_bar);
//         info!("Covariance.......................{}", self.covariance);
//         info!("Pearson r........................{}", self.pearson_r);
//         info!("Slope (Beta).....................{}", self.slope_beta);
//         info!("Estimated Slope (Beta-hat).......{}", self.slope_beta_hat);
//         info!("Intercept (Alpha)................{}", self.intercept_alpha);
//         info!(
//             "Estimated Intercept (Alpha-hat)..{}",
//             self.intercept_alpha_hat
//         );
//         info!(
//             "SE(Beta-hat).....................{}",
//             self.standard_error_of_regression_slope
//         );
//         info!(
//             "SE(alpha-hat)....................{}",
//             self.standard_error_of_regression_intercept
//         );
//         info!(
//             "t-score (coefficient)............{}",
//             self.t_score_coefficient
//         );
//         info!(
//             "t-score (intercept)..............{}",
//             self.t_score_intercept
//         );
//         info!(
//             "Paired Sample t test.............{}",
//             self.paired_sample_t_test
//         );
//         info!(
//             "Ind. Groups t test...............{}",
//             self.independent_groups_t
//         );
//         // info!("Observed Values (Y_i)............{:?}", self.data_y.data);
//         // info!("Fitted Values (Y-hat)............{:?}", self.fitted_values);
//         // info!("Residuals (Y_i - Y-hat)..........{:?}", self.residuals);
//         info!(
//             "Sum of Squared Differences.......{}",
//             self.sum_of_squares_differences
//         );
//         info!(
//             "Sum of Squared Totals............{}",
//             self.sum_of_squares_total
//         );
//         info!(
//             "Sum of Squared Errors............{}",
//             self.sum_of_squares_error
//         );
//         info!(
//             "Explained Sum of Squares.........{}",
//             self.explained_sum_of_squares
//         );
//         info!(
//             "Mean Square Regression...........{}",
//             self.mean_square_regression
//         );
//         info!(
//             "Mean Square Error................{}",
//             self.mean_square_error
//         );
//         info!(
//             "Residual Standard Error..........{}",
//             self.residual_standard_error
//         );
//         info!(
//             "R^2..............................{}",
//             self.coefficient_of_determination
//         );
//         info!(
//             "R^2 adjusted.....................{}",
//             self.coefficient_of_determination_adjusted
//         );
//         info!(
//             "F-statistic......................{}",
//             self.one_way_anova_f_statistic
//         );
//         info!("{}", logging::format_title(""));
//     }
// }
