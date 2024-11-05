use anyhow::{Error, Result};

use charming::element::AxisType;
use charming::{component::Axis, Chart, HtmlRenderer};

pub fn create_chart(x_type: AxisType, y_type: AxisType) -> Result<Chart, Error> {
    let chart = Chart::new()
        .x_axis(Axis::new()
            .type_(x_type))
        .y_axis(Axis::new()
            .type_(y_type));
    Ok(chart)
}

pub fn render_chart(chart: &Chart, title: String, image_width: u64, image_height: u64) -> Result<(), Error> {
    let file_name = title.replace(" ", "-");
    HtmlRenderer::new(title, image_width, image_height).save(&chart, "./graphics/".to_owned() + &*file_name + ".html")?;
    Ok(())
}
