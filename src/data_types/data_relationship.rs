use crate::data_types::data_array::data::{CategoricalDataArray, ContinuousDataArray};
use crate::functions::stats_math::{differences, mean, pooled_variance, variance};
use crate::logging;
use anyhow::{anyhow, Error};
use log::info;

pub trait Statistic {
    fn run_statistic(&mut self) -> anyhow::Result<(), Error>;
    fn print(self);
}

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

    pub t: f64,
}

impl<'a> SingleSampleT<'a> {
    pub fn new(
        name: String,
        description: String,
        data: &'a ContinuousDataArray,
        mu: f64,
    ) -> anyhow::Result<SingleSampleT<'a>, Error> {
        Ok(SingleSampleT {
            name,
            description,
            _n: data.data_array.data.len(),
            _df: data.data_array.data.len() - 1,
            _data: data,
            _mu: mu,
            _variance: data.variance,
            _standard_deviation: data.standard_deviation,
            t: 0.0,
        })
    }
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

    pub t: f64,
}

impl<'a> PairedSamplesT<'a> {
    pub fn new(
        name: String,
        description: String,
        data_x: &'a ContinuousDataArray,
        data_y: &'a ContinuousDataArray,
    ) -> anyhow::Result<PairedSamplesT<'a>, Error> {
        if data_x.data_array.data.len() == data_y.data_array.data.len() {
            Ok(PairedSamplesT {
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
                t: 0.0,
            })
        } else {
            Err(anyhow!("provided data are not of same length"))
        }
    }
}

#[derive(Debug, Clone)]
pub struct IndependentGroupsT<'a> {
    pub name: String,
    pub description: String,
    _level_row_indices: Vec<&'a Vec<usize>>,
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
        categorical_data_array: &'a CategoricalDataArray,
        continuous_data_array: &'a ContinuousDataArray,
    ) -> anyhow::Result<IndependentGroupsT<'a>, Error> {
        let mut new_igt = IndependentGroupsT {
            name,
            description,
            _level_row_indices: vec![],
            _df: 0,
            _categorical_data: categorical_data_array,
            _continuous_data: continuous_data_array,
            _variance_level_1: 0.0,
            _variance_level_2: 0.0,
            _pooled_variance: 0.0,
            _standard_deviation_differences_between_means: 0.0,
            _statistic_run: false,
            t: 0.0,
        };

        new_igt._level_row_indices = new_igt
            ._categorical_data
            .levels
            .iter()
            .map(|x| x.1)
            .collect::<Vec<&'a Vec<usize>>>();

        Ok(new_igt)
    }
}

#[derive(Debug, Clone)]
pub struct ZTest<'a> {
    pub name: String,
    pub n: usize,
    pub df: usize,

    pub data: &'a ContinuousDataArray,

    // provided
    pub mu: f64,
    pub standard_deviation: f64,

    // calculated
    pub z: f64,
}

impl<'a> Statistic for SingleSampleT<'a> {
    fn run_statistic(&mut self) -> anyhow::Result<(), Error> {
        info!("...Calculating 'Single Sample t'...");
        self._n = self._data.data_array.data.len();
        self._df = self._n - 1;
        self.t = (self._data.mean - self._mu) / self._standard_deviation;
        Ok(())
    }

    fn print(self) {
        info!("Single Sample t = {}", self.t)
    }
}

impl<'a> Statistic for PairedSamplesT<'a> {
    fn run_statistic(&mut self) -> anyhow::Result<(), Error> {
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
            self._variance_of_differences = self._sum_of_squares_differences
                / (data_x.len() as f64
                    - if self._data_x.population.unwrap_or_default() {
                        0.0
                    } else {
                        1.0
                    });
            self._s_sub_d_bar = f64::sqrt(self._variance_of_differences);
            self.t = (self._mean_of_differences - 0.0) / self._s_sub_d_bar;

            Ok(())
        } else {
            Err(anyhow!(
                "Data X and Data Y differ in lengths--cannot run 'Paired Sample t'"
            ))
        }
    }
    fn print(self) {
        info!("Paired Sample t = {}", self.t)
    }
}

impl<'a> Statistic for IndependentGroupsT<'a> {
    fn run_statistic(&mut self) -> anyhow::Result<(), Error> {
        if self._categorical_data.levels.keys().len() == 2 {
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

            // info!("Categorical Data: {:#?}", self._categorical_data);
            // info!("Continuous Data: {:#?}", self._continuous_data);

            let filter = |mut i: core::slice::Iter<usize>| {
                let mut next_val = i.next();
                // info!("Starting index: {:?}", next_val);
                return self
                    ._continuous_data
                    .data_array
                    .data
                    .iter()
                    .filter_map(|x| -> Option<f64> {
                        // info!("Index = {}, Value = {}", x.0, x.1);
                        if next_val.is_some() && x.0 == *next_val.unwrap() {
                            next_val = i.next();
                            // info!("Next index: {:?}", next_val);
                            Some(x.1)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<f64>>();
            };
            // info!("All row indices: {:?}", self._level_row_indices);
            let mut iter = self._level_row_indices[0].iter();
            let level_1_continuous_data = filter(iter);
            // info!("Level 1 continuous Data: {:?}", level_1_continuous_data);
            iter = self._level_row_indices[1].iter();
            let level_2_continuous_data = filter(iter);
            // info!("Level 2 continuous Data: {:?}", level_2_continuous_data);

            self._variance_level_1 =
                variance(&level_1_continuous_data, self._continuous_data.population)?;
            self._variance_level_2 =
                variance(&level_2_continuous_data, self._continuous_data.population)?;

            self._pooled_variance = pooled_variance(
                &level_1_continuous_data,
                &level_2_continuous_data,
                Some(self._variance_level_1),
                Some(self._variance_level_2),
            )?;

            self._standard_deviation_differences_between_means = f64::sqrt(
                (self._pooled_variance / self._level_row_indices[0].len() as f64)
                    + (self._pooled_variance / self._level_row_indices[1].len() as f64),
            );

            self.t = (mean(&level_1_continuous_data)? - mean(&level_2_continuous_data)?)
                / self._standard_deviation_differences_between_means;

            self._statistic_run = true;

            Ok(())
        } else {
            Err(anyhow!(
    "A categorical variable with two levels is required to run an independent groups t test"
    ))
        }
    }
    fn print(mut self) {
        if self._statistic_run {
            info!("{}", logging::format_title(&*self.name));
            info!("Description: {}", self.description);
            info!("Level 1: {}", self._categorical_data.data_array.data[0].0);
            info!("Level 2: {}", self._categorical_data.data_array.data[1].0);
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
