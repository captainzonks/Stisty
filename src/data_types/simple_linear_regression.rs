use std::iter::Sum;
use anyhow::{Error, Result};
use charming::element::AxisType;
use log::info;
use crate::data_types::data_array::DataArray;
use crate::error_types::CSVError;
use crate::functions::convert::Convert;
use crate::functions::stats_math::{mean, standard_deviation};
use crate::graphing::{add_line_data, add_scatter_data, create_chart, render_chart, Graph};
use crate::logging;

#[derive(Default, Debug, Clone)]
pub struct SimpleLinearRegression {
    pub name: String,
    pub n: f64,
    pub p: f64, // this will always be 1.0 in a simple linear regression with 1.0 regressor

    pub data_x: DataArray,
    pub data_y: DataArray,

    pub differences: Vec<f64>,
    pub sum_of_differences: f64,
    pub sum_of_squares_of_differences: f64,
    pub variance_of_differences: f64,
    pub s_sub_d_bar: f64,

    pub sum_of_product_of_z_scores: f64,
    pub sum_of_product_of_deviations: f64,

    pub covariance: f64,
    pub pearson_r: f64,

    pub slope_beta: f64, // biased
    pub slope_beta_hat: f64, // unbiased
    pub intercept_alpha: f64, // biased
    pub intercept_alpha_hat: f64, // unbiased
    pub t_score_coefficient: f64, // (from Pearson r) r * sqrt(N - 2) / sqrt(1 - r^2)
    pub t_score_intercept: f64, // intercept / standard error of intercept
    pub paired_sample_t_test: f64,

    pub standard_error_of_regression_slope: f64, // SE(Beta-hat) = sqrt((1/(n-p-1)*MSE)/SSx)
    pub standard_error_of_regression_intercept: f64, // SE(alpha-hat) = SE(Beta-hat) * sqrt((1/n)*sum(x^2))

    pub fitted_values: Vec<f64>, // y-hat
    pub residuals: Vec<f64>, // e_i = y_i - y-hat

    pub sum_of_squares_total: f64, // SST
    pub sum_of_squares_error: f64, // SSE
    pub explained_sum_of_squares: f64, // ESS

    pub mean_square_regression: f64, // MSR = ESS / p, or MSR = ESS in simple linear regression
    pub mean_square_error: f64, // MSE = SSE / (n - p); standard error of the estimate
    pub residual_standard_error: f64, // sqrt((1/(n - p - 1)) * SSE)

    pub coefficient_of_determination: f64, // R^2
    pub coefficient_of_determination_adjusted: f64, // R^2 adjusted

    pub one_way_anova_f_statistic: f64, // Type 1

    // R^2 = proportion of observed y variation that can be explained by the simple linear regression model
}

impl SimpleLinearRegression {
    pub fn new(name: String, data_x: &DataArray, data_y: &DataArray) -> Result<SimpleLinearRegression, Error> {
        let mut new_relationship: SimpleLinearRegression = SimpleLinearRegression::default();
        new_relationship.name = name;
        new_relationship.n = data_x.data.len() as f64;
        new_relationship.p = 1.0; // simple linear regression has only one regressor

        new_relationship.data_x = data_x.clone();
        new_relationship.data_y = data_y.clone();

        // Differences of Data
        let mut iter = new_relationship.data_y.data.iter();
        new_relationship.differences = new_relationship.data_x.data.iter()
            .map(|x| x - iter.next().unwrap())
            .collect();

        // Sum of Differences
        new_relationship.sum_of_differences = new_relationship.differences.iter().sum::<f64>();

        // Sum of Squares of Differences
        new_relationship.sum_of_squares_of_differences = new_relationship.differences.iter()
            .map(|x| f64::powi(*x, 2))
            .sum::<f64>();

        // Variance of Differences
        new_relationship.variance_of_differences = new_relationship.sum_of_squares_of_differences
            / (new_relationship.n
            - if new_relationship.data_x.population.unwrap_or_default() { 0.0 } else { 1.0 });

        // Standard Deviation of the Differences
        new_relationship.s_sub_d_bar = f64::sqrt(new_relationship.variance_of_differences);

        // Paired Sample t Test = (mean(differences) - 0) / (standard deviation of the differences / sqrt(n))
        new_relationship.paired_sample_t_test =
            ((new_relationship.sum_of_differences / new_relationship.n) - 0.0)
                / (new_relationship.s_sub_d_bar / f64::sqrt(new_relationship.n));

        // Sum of Product of Z-Scores
        iter = new_relationship.data_y.z_scores.iter();
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
            / (new_relationship.n
            - if new_relationship.data_x.population.unwrap_or_default() { 0.0 } else { 1.0 });

        // Pearson r = covariance / (data_x's sd * data_y's sd)
        new_relationship.pearson_r = new_relationship.covariance
            / (new_relationship.data_x.standard_deviation * new_relationship.data_y.standard_deviation);

        // t-score for coefficient (from Pearson r) = r * sqrt(N - 2) / sqrt(1 - r^2)
        new_relationship.t_score_coefficient = new_relationship.pearson_r
            * f64::sqrt(new_relationship.n - new_relationship.p - 1.0)
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

        // MSR = ESS / p, or MSR = ESS (since p = 1)
        new_relationship.mean_square_regression = new_relationship.explained_sum_of_squares
            / new_relationship.p;

        // MSE = SSE / n
        new_relationship.mean_square_error = new_relationship.sum_of_squares_error
            / (new_relationship.n - new_relationship.p - 1.0);

        // RSE = sqrt((1/(n - p - 1)) * SSE)
        new_relationship.residual_standard_error =
            f64::sqrt((1.0 / (new_relationship.n - new_relationship.p - 1.0))
                * new_relationship.sum_of_squares_error);

        // SE(Beta-hat) = sqrt((1/(n-p-1))*(MSE/SSx))
        new_relationship.standard_error_of_regression_slope =
            f64::sqrt((1.0 / (new_relationship.n - new_relationship.p - 1.0))
                * (new_relationship.sum_of_squares_error
                / new_relationship.data_x.sum_of_squares));

        // SE(alpha-hat) = SE(Beta-hat) * sqrt((1/n)*sum(x^2))
        new_relationship.standard_error_of_regression_intercept =
            new_relationship.standard_error_of_regression_slope
                * f64::sqrt((1.0 / new_relationship.n)
                * new_relationship.data_x.data.iter()
                .map(|x| f64::powi(*x, 2))
                .collect::<Vec<f64>>().iter().sum::<f64>());

        // t-score for the intercept (alpha / standard error of intercept)
        new_relationship.t_score_intercept = new_relationship.intercept_alpha
            / new_relationship.standard_error_of_regression_intercept;

        // ESS, cheaper method (and perhaps not completely accurate)
        // new_relationship.explained_sum_of_squares = new_relationship.sum_of_squares_total - new_relationship.sum_of_squares_error;

        // coefficient of multiple determination, or R^2 = ESS/SST
        new_relationship.coefficient_of_determination =
            new_relationship.explained_sum_of_squares
                / new_relationship.sum_of_squares_total;

        // R^2 adjusted = 1 - (1 - R^2) * ((n - 1) / (n - p - 1))
        new_relationship.coefficient_of_determination_adjusted =
            1.0 - ((1.0 - new_relationship.coefficient_of_determination) *
                ((new_relationship.n - 1.0) / (new_relationship.n - new_relationship.p - 1.0)));

        // F-statistic, one-way ANOVA Type 1
        new_relationship.one_way_anova_f_statistic = new_relationship.mean_square_regression
            / new_relationship.mean_square_error;


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
        info!("df...............................{}", self.n - self.p - 1.0);
        info!("Data X mean......................{}", self.data_x.mean);
        info!("Data Y mean......................{}", self.data_y.mean);
        info!("Sum of Product of Z-Scores.......{}", self.sum_of_product_of_z_scores);
        info!("Sum of Product of Deviations.....{}", self.sum_of_product_of_deviations);
        info!("Covariance.......................{}", self.covariance);
        info!("Pearson r........................{}", self.pearson_r);
        info!("Slope (Beta).....................{}", self.slope_beta);
        info!("Estimated Slope (Beta-hat).......{}", self.slope_beta_hat);
        info!("Intercept (Alpha)................{}", self.intercept_alpha);
        info!("Estimated Intercept (Alpha-hat)..{}", self.intercept_alpha_hat);
        info!("SE(Beta-hat).....................{}", self.standard_error_of_regression_slope);
        info!("SE(alpha-hat)....................{}", self.standard_error_of_regression_intercept);
        info!("t-score (coefficient)............{}", self.t_score_coefficient);
        info!("t-score (intercept)..............{}", self.t_score_intercept);
        info!("Paired Sample t test.............{}", self.paired_sample_t_test);
        // info!("Observed Values (Y_i)............{:?}", self.data_y.data);
        // info!("Fitted Values (Y-hat)............{:?}", self.fitted_values);
        // info!("Residuals (Y_i - Y-hat)..........{:?}", self.residuals);
        info!("Sum of Squared Totals............{}", self.sum_of_squares_total);
        info!("Sum of Squared Errors............{}", self.sum_of_squares_error);
        info!("Explained Sum of Squares.........{}", self.explained_sum_of_squares);
        info!("Mean Square Regression...........{}", self.mean_square_regression);
        info!("Mean Square Error................{}", self.mean_square_error);
        info!("Residual Standard Error..........{}", self.residual_standard_error);
        info!("R^2..............................{}", self.coefficient_of_determination);
        info!("R^2 adjusted.....................{}", self.coefficient_of_determination_adjusted);
        info!("F-statistic......................{}", self.one_way_anova_f_statistic);
        info!("{}", logging::format_title(""));
    }
}

impl Graph for SimpleLinearRegression {
    fn graph(&self) -> Result<(), Error> {
        let mut data_y_iter = self.data_y.data.clone().into_iter();
        let scatter_data = self.data_x.data.clone().into_iter()
            .map(|x: f64| vec![x, data_y_iter.next().unwrap()])
            .collect::<Vec<Vec<f64>>>();
        data_y_iter = self.fitted_values.clone().into_iter();
        let line_data = self.data_x.data.clone().into_iter()
            .map(|x: f64| vec![x, data_y_iter.next().unwrap()])
            .collect::<Vec<Vec<f64>>>();

        let mut chart = create_chart(AxisType::Value, AxisType::Value)?;
        chart = add_scatter_data(chart, scatter_data)?;
        chart = add_line_data(chart, line_data)?;
        render_chart(chart, String::from("Simple Linear Graph"), 1000, 800)?;

        Ok(())
    }
}