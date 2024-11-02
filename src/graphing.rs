use anyhow::{Result, Error};

use charming::{component::Axis, series::Scatter, series::Line, Chart, HtmlRenderer};
use charming::element::AxisType;

pub fn create_chart(x_type: AxisType, y_type: AxisType) -> Result<Chart, Error> {
    let chart = Chart::new()
        .x_axis(Axis::new()
            .type_(x_type))
        .y_axis(Axis::new()
            .type_(y_type));
    Ok(chart)
}

pub fn add_scatter_data(mut chart: Chart, data: Vec<Vec<f64>>) -> Result<Chart, Error> {
    chart = chart.series(Scatter::new().symbol_size(20).data(data));
    Ok(chart)
}

pub fn add_line_data(mut chart: Chart, line_data: Vec<Vec<f64>>) -> Result<Chart, Error> {
    chart = chart.series(Line::new().data(line_data));
    Ok(chart)
}

pub fn render_chart(mut chart: Chart, title: String, image_width: u64, image_height: u64) -> Result<(), Error> {
    HtmlRenderer::new(String::from(title), image_width, image_height).save(&mut chart, "./graphics/scatter.html")?;
    Ok(())
}

pub trait Graph {
    fn graph(&self) -> Result<(), Error>;
}
