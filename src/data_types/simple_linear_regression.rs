use std::iter::Sum;
use anyhow::{Error, Result};
use log::info;
use crate::data_types::data_array::DataArray;
use crate::error_types::CSVError;
use crate::functions::convert::Convert;
use crate::functions::stats_math::mean;
use crate::logging;

#[derive(Default, Debug, Clone)]
pub struct SimpleLinearRegression {
    pub name: String,
    pub n: i32,
    pub p: i32, // this will always be 1 in a simple linear regression with 1 regressor

    pub data_x: DataArray,
    pub data_y: DataArray,

    pub sum_of_product_of_z_scores: f64,
    pub sum_of_product_of_deviations: f64,

    pub covariance: f64,
    pub pearson_r: f64,
    pub t_score: f64,

    pub slope_beta: f64, // biased
    pub slope_beta_hat: f64, // unbiased
    pub intercept_alpha: f64, // biased
    pub intercept_alpha_hat: f64, // unbiased

    pub standard_error_of_regression_slope: f64, // SE(Beta-hat) = sqrt((1/(n-p-1)*MSE)/SSx)
    pub standard_error_of_regression_intercept: f64, // SE(alpha-hat) = SE(Beta-hat) * sqrt((1/n)*sum(x^2))

    // pub observed_values: Vec<f64>, // y_i
    pub fitted_values: Vec<f64>, // y-hat
    pub residuals: Vec<f64>, // e_i = y_i - y-hat

    pub sum_of_squares_total: f64, // SST
    pub sum_of_squares_error: f64, // SSE
    pub explained_sum_of_squares: f64, // ESS

    pub mean_square_error: f64, // MSE = SSE / (n - p); standard error of the estimate

    pub coefficient_of_multiple_determination: f64, // R^2
    pub coefficient_of_multiple_determination_adjusted: f64, // R^2 adjusted

    // R^2 = proportion of observed y variation that can be explained by the simple linear regression model
}

impl SimpleLinearRegression {
    pub fn new(name: String, data_x: &DataArray, data_y: &DataArray) -> Result<SimpleLinearRegression, Error> {
        let mut new_relationship: SimpleLinearRegression = SimpleLinearRegression::default();
        new_relationship.name = name;
        new_relationship.n = data_x.data.len() as i32;
        new_relationship.p = 1; // simple linear regression has only one regressor

        new_relationship.data_x = data_x.clone();
        new_relationship.data_y = data_y.clone();

        // Sum of Product of Z-Scores
        let mut iter = new_relationship.data_y.z_scores.iter();
        new_relationship.sum_of_product_of_z_scores = new_relationship.data_x.z_scores.iter()
            .map(|x_z| x_z * iter.next().unwrap())
            .sum::<f64>();

        // Sum of Product of Deviations
        iter = new_relationship.data_y.deviations.iter();
        new_relationship.sum_of_product_of_deviations = new_relationship.data_x.deviations.iter()
            .map(|dev_x| dev_x * iter.next().unwrap())
            .sum::<f64>();

        // Covariance = (sum(data_x's deviations * data_y's deviations)) / (N (- 1, for Bessel's Correction))
        new_relationship.covariance = new_relationship.sum_of_product_of_deviations
            / (new_relationship.data_x.data.len() as f64
            - if new_relationship.data_x.population.unwrap_or_default() { 0.0 } else { 1.0 });

        // Pearson r = covariance / (data_x's sd * data_y's sd)
        new_relationship.pearson_r = new_relationship.covariance
            / (new_relationship.data_x.standard_deviation * data_y.standard_deviation);

        // t-score (from Pearson r) = r * sqrt(N - 2) / sqrt(1 - r^2)
        new_relationship.t_score = new_relationship.pearson_r
            * f64::sqrt(new_relationship.data_x.data.len() as f64 - 2.0)
            / f64::sqrt(1.0 - f64::powi(new_relationship.pearson_r, 2));

        // y-hat = beta(x) + alpha
        // x = (y-hat - alpha) / beta
        // beta = (y-hat - alpha) / x
        // alpha = y-hat - (beta)x

        new_relationship.slope_beta = new_relationship.covariance / new_relationship.data_x.variance;

        new_relationship.intercept_alpha = new_relationship.data_y.mean
            - new_relationship.slope_beta * new_relationship.data_x.mean;

        // beta_hat = sum((x_i - x-bar)(y - y-bar)) / sum((x_i - x-bar)^2)
        new_relationship.slope_beta_hat = new_relationship.sum_of_product_of_deviations
            / new_relationship.data_x.sum_of_squares;

        // alpha_hat = y-bar - (beta_hat * x-bar)
        new_relationship.intercept_alpha_hat = new_relationship.data_y.mean
            - (new_relationship.slope_beta_hat * new_relationship.data_x.mean);

        // calculate fitted
        for datum in new_relationship.data_x.data.iter() {
            new_relationship.fitted_values.push(new_relationship.get_y_hat(*datum));
        }

        // calculate and collect residuals
        let mut fitted_iter = new_relationship.fitted_values.iter();
        new_relationship.residuals = new_relationship.data_y.data.iter()
            .map(|y_i| y_i - fitted_iter.next().unwrap())
            .collect();

        // SST = sum((y_i - y_mean)^2)
        new_relationship.sum_of_squares_total = new_relationship.data_y.data.iter()
            .map(|y_i| f64::powi(y_i - new_relationship.data_y.mean, 2))
            .sum::<f64>();

        // SSE = sum(residual^2)
        new_relationship.sum_of_squares_error = new_relationship.residuals.iter()
            .map(|residual| f64::powi(*residual, 2))
            .sum::<f64>();

        // ESS = sum((fitted_y - y_mean)^2)
        new_relationship.explained_sum_of_squares = new_relationship.fitted_values.iter()
            .map(|fitted_y| f64::powi(fitted_y - new_relationship.data_y.mean, 2))
            .sum::<f64>();

        // calculate the mean square error
        // MSE = SSE / (n - p - 1)
        new_relationship.mean_square_error = new_relationship.sum_of_squares_error
            / (new_relationship.n - new_relationship.p - 1) as f64;

        // SE(Beta-hat) = sqrt((1/(n-p-1)*MSE)/SSx)
        new_relationship.standard_error_of_regression_slope =
            f64::sqrt(((1 / new_relationship.n - new_relationship.p - 1) as f64
                * new_relationship.mean_square_error)
                / new_relationship.data_x.sum_of_squares);

        // SE(alpha-hat) = SE(Beta-hat) * sqrt((1/n)*sum(x^2))
        new_relationship.standard_error_of_regression_intercept =
            new_relationship.standard_error_of_regression_slope
                * f64::sqrt((1 / new_relationship.n) as f64
                * new_relationship.data_x.data.iter().sum::<f64>());


        // ESS, cheaper method (and perhaps not completely accurate)
        // new_relationship.explained_sum_of_squares = new_relationship.sum_of_squares_total - new_relationship.sum_of_squares_error;

        // coefficient of multiple determination, or R^2 (ESS/SST)
        new_relationship.coefficient_of_multiple_determination = new_relationship.explained_sum_of_squares
            / new_relationship.sum_of_squares_total;

        // R^2 adjusted = 1 - ((n - 1) / (n - p - 1)) * (1 - R^2)
        new_relationship.coefficient_of_multiple_determination_adjusted =
            1.0 - ((new_relationship.n - 1) / (new_relationship.n - new_relationship.p - 1)) as f64
                * (new_relationship.explained_sum_of_squares / new_relationship.sum_of_squares_total);


        Ok(new_relationship)
    }

    pub fn get_y_hat(&self, x_value: f64) -> f64 {
        self.slope_beta * x_value + self.intercept_alpha
    }

    pub fn get_x(&self, y_value: f64) -> f64 {
        (y_value - self.intercept_alpha) / self.slope_beta
    }

    pub fn get_intercept_alpha(&self, y_value: f64, x_value: f64) -> f64 {
        y_value - self.intercept_alpha * x_value
    }

    pub fn get_slope_beta(&self, y_value: f64, x_value: f64) -> f64 {
        (y_value - self.intercept_alpha) / x_value
    }

    pub fn print_relationship(&self) {
        info!("{}", logging::format_title(&*self.name));
        info!("n................................{}", self.n);
        info!("p................................{}", self.p);
        info!("df...............................{}", self.n - self.p - 1);
        info!("Data X mean......................{}", self.data_x.mean);
        info!("Data Y mean......................{}", self.data_y.mean);
        info!("Sum of Product of Z-Scores.......{}", self.sum_of_product_of_z_scores);
        info!("Sum of Product of Deviations.....{}", self.sum_of_product_of_deviations);
        info!("Covariance.......................{}", self.covariance);
        info!("Pearson r........................{}", self.pearson_r);
        info!("t-score..........................{}", self.t_score);
        info!("Slope (Beta).....................{}", self.slope_beta);
        info!("Estimated Slope (Beta-hat).......{}", self.slope_beta_hat);
        info!("Intercept (Alpha)................{}", self.intercept_alpha);
        info!("Estimated Intercept (Alpha-hat)..{}", self.intercept_alpha_hat);
        info!("SE(Beta-hat).....................{}", self.standard_error_of_regression_slope);
        info!("SE(alpha-hat)....................{}", self.standard_error_of_regression_intercept);
        info!("Observed Values (Y_i)............{:?}", self.data_y.data);
        info!("Fitted Values (Y-hat)............{:?}", self.fitted_values);
        info!("Residuals (Y_i - Y-hat)..........{:?}", self.residuals);
        info!("Sum of Squared Totals............{}", self.sum_of_squares_total);
        info!("Sum of Squared Errors............{}", self.sum_of_squares_error);
        info!("Explained Sum of Squares.........{}", self.explained_sum_of_squares);
        info!("Mean Square Error................{}", self.mean_square_error);
        info!("R^2..............................{}", self.coefficient_of_multiple_determination);
        info!("R^2 adjusted.....................{}", self.coefficient_of_multiple_determination_adjusted);
        info!("{}", logging::format_title(""));
    }
}
