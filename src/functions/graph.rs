use crate::data_types::simple_linear_regression::SimpleLinearRegression;
use anyhow::{Error, Result};
use charming::{Chart, HtmlRenderer};
use charming::component::Axis;
use charming::element::AxisType;
use charming::series::{Line, Scatter};
use log::info;

macro_rules! data_points {
    ($x: expr, $y: expr) => {
        $x.map(|x: f64| vec![x, $y.next().unwrap()])
            .collect::<Vec<Vec<f64>>>()
    };
}

pub trait Graph<T> {
    fn graph(data: &T) -> Result<(), Error>;

    fn create_chart(x_type: AxisType, y_type: AxisType) -> Result<Chart, Error> {
        let chart = Chart::new()
            .x_axis(Axis::new()
                .type_(x_type))
            .y_axis(Axis::new()
                .type_(y_type));
        Ok(chart)
    }

    fn render_chart(chart: &Chart, title: String, image_width: u64, image_height: u64) -> Result<(), Error> {
        let file_name = title.replace(" ", "-");
        HtmlRenderer::new(title, image_width, image_height)
            .save(&chart, "./graphics/".to_owned() + &*file_name + ".html")?;
        Ok(())
    }
}

impl Graph<SimpleLinearRegression> for Scatter {
    fn graph(data: &SimpleLinearRegression) -> Result<(), Error> {
        let mut chart = Scatter::create_chart(AxisType::Value, AxisType::Value)?;
        let data_x_iter = data.data_x.data.clone().into_iter();
        let mut data_y_iter = data.data_y.data.clone().into_iter();

        let file_name = String::from(data.name.as_str().to_owned() + "_scatter");

        chart = chart.series(Scatter::new().symbol_size(10).data(data_points!(data_x_iter, data_y_iter)));
        info!("Generating and saving scatter plot as './graphics/{}'.html", file_name);
        Scatter::render_chart(&chart, file_name, 1000, 800)?;

        Ok(())
    }
}

impl Graph<SimpleLinearRegression> for Line {
    fn graph(data: &SimpleLinearRegression) -> Result<(), Error> {
        let mut chart = Line::create_chart(AxisType::Value, AxisType::Value)?;
        let data_x_iter = data.data_x.data.clone().into_iter();
        let mut data_y_iter = data.fitted_values.clone().into_iter();

        let file_name = String::from(data.name.as_str().to_owned() + "_line");

        chart = chart.series(Line::new().symbol_size(10).data(data_points!(data_x_iter, data_y_iter)));
        info!("Generating and saving line graph as './graphics/{}'.html", file_name);
        Line::render_chart(&chart, file_name, 1000, 800)?;

        Ok(())
    }
}
