use crate::data_types::simple_linear_regression::SimpleLinearRegression;
use crate::graphing::{create_chart, render_chart};
use anyhow::{Error, Result};
use charming::element::AxisType;
use charming::series::{Line, Scatter};

macro_rules! data_points {
    ($x: expr, $y: expr) => {
        $x.map(|x: f64| vec![x, $y.next().unwrap()])
            .collect::<Vec<Vec<f64>>>()
    };
}

pub trait Graph<T> {
    fn graph(data: &T) -> Result<(), Error>;
}

impl Graph<SimpleLinearRegression> for Scatter {
    fn graph(data: &SimpleLinearRegression) -> Result<(), Error> {
        let mut chart = create_chart(AxisType::Value, AxisType::Value)?;
        let data_x_iter = data.data_x.data.clone().into_iter();
        let mut data_y_iter = data.data_y.data.clone().into_iter();

        let file_name = String::from(data.name.as_str().to_owned() + "_scatter");

        chart = chart.series(Scatter::new().symbol_size(10).data(data_points!(data_x_iter, data_y_iter)));
        render_chart(&chart, file_name, 1000, 800)?;

        Ok(())
    }
}

impl Graph<SimpleLinearRegression> for Line {
    fn graph(data: &SimpleLinearRegression) -> Result<(), Error> {
        let mut chart = create_chart(AxisType::Value, AxisType::Value)?;
        let data_x_iter = data.data_x.data.clone().into_iter();
        let mut data_y_iter = data.fitted_values.clone().into_iter();

        let file_name = String::from(data.name.as_str().to_owned() + "_line");

        chart = chart.series(Line::new().symbol_size(10).data(data_points!(data_x_iter, data_y_iter)));
        render_chart(&chart, file_name, 1000, 800)?;

        Ok(())
    }
}
