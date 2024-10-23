use anyhow::{Error, Result};
use log::info;
use crate::data_types::data_array::DataArray;
use crate::data_types::relationship::Relationship;
use crate::logging;

#[derive(Default, Debug)]
pub struct MultipleRegression {
    pub name: String,
    pub n: i32, // total count of all observations
    pub p: i32, // total count of predictor variables
    pub x_data_arrays: Vec<DataArray>,
    pub y_data_array: DataArray,
    pub data_relationships: Vec<Relationship>,
    pub group_means: Vec<f64>,
    pub grand_mean: f64,
    pub sum_of_squares_between_groups: f64, // SSB
    pub sum_of_squares_within_groups: f64, // SSW
    pub sum_of_squared_residuals: f64, // SSR
    pub sum_of_squares_total: f64, // SST
    pub explained_sum_of_squares: f64, // ESS (or sum of squares due to regression)
    pub degrees_of_freedom_between_groups: i32, // dfB
    pub degrees_of_freedom_within_groups: i32, // dfW
    pub mean_square_between_groups: f64, // MSB
    pub mean_square_error: f64, // MSE = SSE / (n - p)
    pub mean_square_regression: f64, // MSR
    pub root_mean_square_error: f64, // RMSE
    pub f_type_1: f64,
    pub coefficient_of_multiple_determination: f64, // R^2
    pub coefficient_of_multiple_determination_adjusted: f64, // R^2 adjusted

    pub explained_variance: f64, // between-group variability
    pub unexplained_variance: f64, // within-group variability
    pub one_way_anova_f_test: f64, //

    sum_of_all_data_points_in_all_groups: f64,

    // Sum of Squares Between Groups (SSB): representing the variability between different groups
    // in the data; it is often denoted as "SSB" (Sum of Squares Between) and is a key component in
    // determining if there are significant differences between groups in an analysis of variance test

    // Degrees of Freedom Between Groups (dfB): the variation between group means
    // Degrees of Freedom Within Groups (dfW): the variation within each group around their own mean

    // Mean Square Between (MSB): estimates the average variation among the group means,
    // allowing you to compare it to the variation within groups to determine if there are
    // statistically significant differences between the groups

    // Mean Square Error (MSE): essentially representing the "unexplained variance" within the data,
    // used to compare the variability between groups and determine if significant differences exist
    // between them
}

impl MultipleRegression {
    pub fn new(name: String, y_data_array: &DataArray, x_data_arrays: Vec<&DataArray>) -> Result<MultipleRegression, Error> {
        let mut new_multiple_regression: MultipleRegression = MultipleRegression::default();
        new_multiple_regression.name = name;
        new_multiple_regression.x_data_arrays = x_data_arrays.clone().into_iter()
            .map(|data_array: &DataArray| data_array.clone()).collect();
        new_multiple_regression.y_data_array = y_data_array.clone();
        new_multiple_regression.p = x_data_arrays.len() as i32;

        // create an array of relationships of all x data to y data
        for x_data_array in new_multiple_regression.x_data_arrays.iter() {
            new_multiple_regression.data_relationships.push(Relationship::new(
                String::from(x_data_array.name.clone() + " vs " + new_multiple_regression.y_data_array.name.as_str()),
                &x_data_array,
                &new_multiple_regression.y_data_array,
                None)?
            );
        }

        // ANOVA table calculations:

        for (i, data_array) in new_multiple_regression.x_data_arrays.iter().enumerate() {
            // collect all the means
            new_multiple_regression.group_means.push(data_array.mean.clone());

            // keep tally of all data points
            new_multiple_regression.n += data_array.data.len() as i32;

            // reverse the means back into sums for each data array
            new_multiple_regression.sum_of_all_data_points_in_all_groups += new_multiple_regression.group_means.get(i).unwrap()
                * new_multiple_regression.x_data_arrays.get(i).unwrap().data.len() as f64;
        }

        // calculate the grand mean by dividing the sum of all data across arrays by the total data points across arrays
        new_multiple_regression.grand_mean = new_multiple_regression.sum_of_all_data_points_in_all_groups
            / new_multiple_regression.n as f64;

        // SSE (or SSR) = sum of squared residuals
        for relationship in new_multiple_regression.data_relationships.iter() {
            for residual in relationship.residuals.iter() {
                new_multiple_regression.sum_of_squared_residuals +=
                    f64::powi(*residual, 2);
            }
            // ESS = sum of squares of fitted values minus the y_mean
            for fitted in relationship.fitted_values.iter() {
                new_multiple_regression.explained_sum_of_squares +=
                    f64::powi(fitted - relationship.data_y.mean, 2);
            }
            // SST = sum of squares of observed values minus the y_mean
            for observed in relationship.observed_values.iter() {
                new_multiple_regression.sum_of_squares_total +=
                    f64::powi(observed - relationship.data_y.mean, 2);
            }
        }

        for data_array in new_multiple_regression.x_data_arrays.iter() {
            // sum of squares between groups (SSB) = sum(n(mean - grand_mean)^2)
            new_multiple_regression.sum_of_squares_between_groups +=
                f64::powi(data_array.mean - new_multiple_regression.grand_mean, 2)
                    * data_array.data.len() as f64;

            // for datum in data_array.data.iter() {
            //     // sum of squares total (each datum - grand mean, squared, then summed) (SST)
            //     // (METHOD 1; probably slower and more intensive)
            //     // new_multiple_regression.sum_of_squares_total += f64::powi(datum - new_multiple_regression.grand_mean, 2);
            //
            // }
        }

        // Sum of Squared Total (SST) = SSB + SSE (true for simple linear models, but not always)
        // new_multiple_regression.sum_of_squares_total = new_multiple_regression.sum_of_squares_between_groups
        //     + new_multiple_regression.sum_of_squared_residuals;

        // SSR = SST - SSE (true for simple linear models, but not always)
        // new_multiple_regression.sum_of_squares_regression = new_multiple_regression.sum_of_squares_total
        //     - new_multiple_regression.sum_of_squared_residuals;

        // Degrees of freedom between groups (dfB): Number of groups - 1
        new_multiple_regression.degrees_of_freedom_between_groups = new_multiple_regression.p - 1;

        // Degrees of freedom within groups (dfW): Total number of data points - Total number of groups
        // (-1, if an intercept is being used (make this a global switch or something?))
        new_multiple_regression.degrees_of_freedom_within_groups = new_multiple_regression.n
            - new_multiple_regression.p - 1;

        // Mean Square Between Groups (MSB): SSB / dfB
        new_multiple_regression.mean_square_between_groups = new_multiple_regression.sum_of_squares_between_groups
            / new_multiple_regression.degrees_of_freedom_between_groups as f64;

        // Mean Square Error (MSE): SSE / dfW
        new_multiple_regression.mean_square_error = new_multiple_regression.sum_of_squared_residuals
            / new_multiple_regression.degrees_of_freedom_within_groups as f64;

        // Mean Square Regression: dividing the regression sum of squares by its degrees of freedom

        // Root Mean Square Error: SSE / n
        new_multiple_regression.root_mean_square_error = new_multiple_regression.sum_of_squared_residuals
            / new_multiple_regression.n as f64;

        // F = MSB / MSE
        new_multiple_regression.f_type_1 = new_multiple_regression.mean_square_between_groups
            / new_multiple_regression.mean_square_error;

        // Interpreting the ANOVA table:
        // F-statistic: Compare the calculated F-statistic to a critical value from an F-distribution table
        // based on your chosen significance level and degrees of freedom. If the calculated F-statistic is
        // larger than the critical value, you can reject the null hypothesis and conclude that there are
        // significant differences between groups.

        // R^2, coefficient of multiple determination = (1 - (SSR/SST))
        new_multiple_regression.coefficient_of_multiple_determination = 1.0
            - (new_multiple_regression.sum_of_squared_residuals
            / new_multiple_regression.sum_of_squares_total
        );

        // R^2 adjusted = 1 - ((n - 1) / (n - (p + 1)) * (1 - R^2)
        new_multiple_regression.coefficient_of_multiple_determination_adjusted =
            1.0 - ((new_multiple_regression.n - 1) / (new_multiple_regression.n - new_multiple_regression.p - 1)) as f64
                * (1.0 - new_multiple_regression.coefficient_of_multiple_determination);

        Ok(new_multiple_regression)
    }

    pub fn print_multiple_regression(&self) {
        info!("{}", logging::format_title(&*self.name));
        info!("Total Variables...............{}", self.x_data_arrays.len());
        info!("Group Means...................{:?}", self.group_means);
        info!("Grand Mean....................{}", self.grand_mean);
        info!("SST...........................{}", self.sum_of_squares_total);
        info!("SSB...........................{}", self.sum_of_squares_between_groups);
        info!("SSE...........................{}", self.sum_of_squared_residuals);
        info!("ESS...........................{}", self.explained_sum_of_squares);
        info!("dfB...........................{}", self.degrees_of_freedom_between_groups);
        info!("dfW...........................{}", self.degrees_of_freedom_within_groups);
        info!("MSB...........................{}", self.mean_square_between_groups);
        info!("MSE...........................{}", self.mean_square_error);
        info!("MSR...........................{}", self.mean_square_regression);
        info!("RMSD..........................{}", self.root_mean_square_error);
        info!("F Type 1......................{}", self.f_type_1);
        info!("R^2...........................{}", self.coefficient_of_multiple_determination);
        info!("R^2 adjusted..................{}", self.coefficient_of_multiple_determination_adjusted);
        info!("{}", logging::format_title(""));
        // info!("ADDITIONAL DEBUG INFO");
        // info!("Total Points in all Data.....................{}", self.n);
        // info!("Sum of All Data in all Groups..................{}", self.sum_of_all_data_points_in_all_groups);
    }
}

/*
Type I, also called “sequential” sum of squares:

anova_type_1 = MSB / MSE

SS(A) for factor A.

SS(B | A) for factor B.

SS(AB | B, A) for interaction AB.

This tests the main effect of factor A, followed by the main effect of
factor B after the main effect of A, followed by the interaction effect AB after the main effects.

Because of the sequential nature and the fact that the two main factors are tested in a particular order,
this type of sums of squares will give different results for unbalanced data depending on which main
effect is considered first.

For unbalanced data, this approach tests for a difference in the weighted marginal means.
In practical terms, this means that the results are dependent on the realized sample sizes, namely
the proportions in the particular data set. In other words, it is testing the first factor without
controlling for the other factor.

Note that this is often not the hypothesis that is of interest when dealing with unbalanced data.
 */

/*
Type II:

SS(A | B) for factor A.

SS(B | A) for factor B.

This type tests for each main effect after the other main effect.

Note that no significant interaction is assumed (in other words, you should test for interaction
first (SS(AB | A, B)) and only if AB is not significant, continue with the analysis for main effects).

If there is indeed no interaction, then type II is statistically more powerful than type III.

Computationally, this is equivalent to running a type I analysis with different orders of the
factors, and taking the appropriate output (the second, where one main effect is run after the other,
in the example above).
 */

/*
Type III:

S(A | B, AB) for factor A.

SS(B | A, AB) for factor B.

This type tests for the presence of a main effect after the other main effect and interaction.
This approach is therefore valid in the presence of significant interactions.

However, it is often not interesting to interpret a main effect if interactions are present
(generally speaking, if a significant interaction is present, the main effects should not be
further analysed).

If the interactions are not significant, type II gives a more powerful test.
 */

/*
Q = quantile function

Left-tailed test:
(−∞,Q(α)]

Right-tailed test:
[Q(1−α),∞)

Two-tailed test:
(−∞,Q(α/2)] ∪ [Q(1−α/2),∞)
 */

/*
u = quantile function of the standard normal distribution

Left-tailed Z critical value:
u(α)

Right-tailed Z critical value:
u(1−α)

Two-tailed Z critical value:
±u(1−α/2)
 */

/*
Qt,d = quantile function of the t distribution with d degrees of freedom

Left-tailed t critical value:
Qt,d(α)

Right-tailed t critical value:
Qt,d(1−α)

Two-tailed t critical values:
±Qt,d(1−α/2)
 */