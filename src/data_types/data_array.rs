pub mod data {
    use crate::logging;
    use log::info;
    use std::collections::HashMap;

    pub(in crate::data_types::data_array) mod continuous {
        #[derive(Clone, Default, Debug)]
        pub struct DataArray {
            pub data: Vec<(usize, f64)>,
        }
    }

    pub(in crate::data_types::data_array) mod categorical {
        #[derive(Clone, Debug, Default)]
        pub struct DataArray<'a> {
            pub data: Vec<(usize, &'a String)>,
        }
    }

    pub trait Data {
        type DataArray;
        fn print(&self);
    }

    #[derive(Clone, Debug, Default)]
    pub struct ContinuousDataArray {
        pub data_array: continuous::DataArray,
        pub column_index: usize,
        pub name: String,
        pub population: Option<bool>,
        pub n: usize,
        pub mean: f64,
        pub sum_of_squares: f64,
        pub deviations: Vec<f64>,
        pub variance: f64,
        pub standard_deviation: f64,
        pub z_scores: Vec<f64>,
    }

    impl Data for ContinuousDataArray {
        type DataArray = continuous::DataArray;

        fn print(&self) {
            info!("{}", logging::format_title(&*self.name));
            info!("Data Type.....................Continuous",);
            info!("Column Index..................{}", self.column_index);
            // debug!("Data: {:?}", &self.data);
            info!("N.............................{}", self.n);
            info!(
                "Population....................{}",
                self.population.unwrap_or_default()
            );
            info!("Mean..........................{}", self.mean);
            info!("Sum of Squares................{}", self.sum_of_squares);
            // debug!("Deviations: {:?}", self.deviations.clone().unwrap_or_default());
            info!("Variance......................{}", self.variance);
            info!("Standard deviation............{}", self.standard_deviation);
            // debug!("Z-Scores: {:?}", self.z_scores.clone().unwrap_or_default());
        }
    }

    #[derive(Clone, Debug)]
    pub struct CategoricalDataArray<'a> {
        pub data_array: categorical::DataArray<'a>,
        pub column_index: usize,
        pub name: String,
        pub population: Option<bool>,
        pub n: usize,
        pub levels: HashMap<&'a String, Vec<usize>>,
    }

    impl CategoricalDataArray<'_> {
        pub fn retrieve_level_indices(&self, level_name: String) -> Vec<&usize> {
            self.levels
                .iter()
                .filter_map(|(key, indices)| {
                    if level_name == **key {
                        Some(indices)
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<&usize>>()
        }

        // pub fn retrieve_level_and_indices(&self, level_name: String) -> Vec<(&usize, &String)> {
        //     let indices = self.retrieve_level_indices(level_name);
        //     let mut iter = indices.into_iter();
        //     let mut index = iter.next();
        //     self.data_array
        //         .data
        //         .iter()
        //         .filter_map(|(key, value)| {
        //             if index.is_some() && *key == *index.unwrap() {
        //                 index = iter.next();
        //                 Some((key, *value))
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect::<Vec<(&usize, &String)>>()
        // }
    }

    impl<'a> Data for CategoricalDataArray<'a> {
        type DataArray = categorical::DataArray<'a>;

        fn print(&self) {
            info!("{}", logging::format_title(&*self.name));
            info!("Data Type.....................Categorical",);
            info!("Column Index..................{}", self.column_index);
            // debug!("Data: {:?}", &self.data);
            info!("N.............................{}", self.n);
            info!(
                "Population....................{}",
                self.population.unwrap_or_default()
            );
            info!("Levels........................{:#?}", self.levels);
        }
    }
}

pub mod sample_data {
    use super::data::categorical::DataArray;
    use super::data::{CategoricalDataArray, ContinuousDataArray, Data};
    use anyhow::{Error, Result};
    use std::marker::PhantomData;

    #[derive(Clone, Debug)]
    pub struct SampleData<D: Data> {
        _data_type: PhantomData<D>,
    }

    impl SampleData<ContinuousDataArray> {
        pub fn new(
            name: String,
            data: &Vec<f64>,
            column_index: usize,
            pop: Option<bool>,
        ) -> Result<ContinuousDataArray, Error> {
            let mut new_data_array: ContinuousDataArray = Default::default();

            new_data_array.name = name;
            new_data_array.column_index = column_index;
            new_data_array.n = data.len();

            // collect into a vector of tuple (row_num, datum), where rows start at 1 (header is 0)
            new_data_array.data_array.data = data
                .iter()
                .enumerate()
                .map(|x| -> Result<(usize, f64), Error> { Ok((x.0, *x.1)) })
                .collect::<Result<Vec<(usize, f64)>, Error>>()?;

            // establishes if we need to adjust for sample or pop later for variance calculations
            new_data_array.population = pop;

            // mean = sum(x_i) / N
            new_data_array.mean = new_data_array
                .data_array
                .data
                .iter()
                .map(|x| x.1) // extract datum
                .sum::<f64>()
                / new_data_array.data_array.data.len() as f64;

            // ss = sum((x_i - mean)^2)
            new_data_array.sum_of_squares = new_data_array
                .data_array
                .data
                .iter()
                .map(|x| f64::powi(x.1 - new_data_array.mean, 2))
                .sum::<f64>();

            // deviation = x - mean
            new_data_array.deviations = new_data_array
                .data_array
                .data
                .iter()
                .map(|x| x.1 - new_data_array.mean)
                .collect();

            // s^2 = ss / (N - 1)
            // N for pop (true), N-1 for sample (default = false)
            new_data_array.variance = new_data_array.sum_of_squares
                / (new_data_array.data_array.data.len() as f64
                    - if new_data_array.population.unwrap_or_default() {
                        0.0
                    } else {
                        1.0
                    });

            // s = sqrt(s^2)
            new_data_array.standard_deviation = f64::sqrt(new_data_array.variance);

            // z = x / s
            new_data_array.z_scores = new_data_array
                .data_array
                .data
                .iter()
                .map(|x| x.1 / new_data_array.standard_deviation)
                .collect();

            // pub fn get_probability_density(&self, x: f64) -> Result<f64, Error> {
            //     let fraction = 1.0 / f64::sqrt(2.0 * PI * self.variance);
            //     let e_exponential = E.powf(-f64::powi((x - self.mean), 2) / (2.0 * self.variance));
            //     Ok(fraction * e_exponential)
            // }

            // raw = deviation + mean
            // pub fn get_raw_scores_from_deviations(&self) -> Result<Vec<f64>, Error> {
            //     Ok(self.deviations.iter().map(|x| *x + self.mean).collect())
            // }

            // pub fn get_single_t(&self, mu: f64) -> Result<f64, Error> {
            //     Ok((self.mean - mu) / (self.standard_deviation / f64::sqrt(self.data.len() as f64)))
            // }

            Ok(new_data_array)
        }
    }

    impl<'a> SampleData<CategoricalDataArray<'a>> {
        pub fn new(
            name: String,
            data: &'a Vec<String>,
            column_index: usize,
            population: Option<bool>,
        ) -> Result<CategoricalDataArray, Error> {
            let mut new_data_array: CategoricalDataArray = CategoricalDataArray {
                data_array: DataArray {
                    data: Vec::with_capacity(data.len()),
                },
                column_index,
                name,
                population,
                n: data.len(),
                levels: Default::default(),
            };

            new_data_array.data_array.data = data
                .iter()
                .enumerate()
                .map(|x| -> Result<(usize, &'a String), Error> {
                    new_data_array.levels.entry(x.1).or_insert(vec![]).push(x.0);
                    Ok((x.0, &*x.1))
                })
                .collect::<Result<Vec<(usize, &'a String)>, _>>()?;

            Ok(new_data_array)
        }
    }
}
