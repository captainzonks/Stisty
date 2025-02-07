// use std::any::Any;
// use crate::data_types::data_relationship::DataRelationship;
// use anyhow::{Error, Result};
// use charming::{Chart, HtmlRenderer};
// use charming::component::{Axis, DataZoom, DataZoomType, Feature, Restore, SaveAsImage, SaveAsImageType, Toolbox, ToolboxDataZoom};
// use charming::element::{AxisPointer, AxisPointerLink, AxisPointerType, AxisType, Formatter, Tooltip, Trigger};
// use charming::series::{Line, Scatter};
// use log::info;
//
// macro_rules! data_points {
//     ($x: expr, $y: expr) => {{
//         let ret_vec: Vec<Vec<f64>> = $x
//         .map(|x| vec![f64::from(x), f64::from($y.next().unwrap())])
//             .collect::<Vec<Vec<f64>>>();
//         ret_vec
//     }};
// }
//
// pub trait Graph<T> {
//     fn graph(data: &T) -> Result<(), Error>;
//
//     fn create_chart(x_type: AxisType, y_type: AxisType) -> Result<Chart, Error> {
//         let chart = Chart::new()
//             .x_axis(Axis::new()
//                 .type_(x_type))
//             .x_axis(Axis::new()
//                 .type_(AxisType::Value)
//                 .inverse(true))
//             .y_axis(Axis::new()
//                 .type_(y_type)
//                 .id("test"))
//             .y_axis(Axis::new()
//                 .type_(AxisType::Value)
//                 .id("other")
//                 .inverse(true))
//             .tooltip(
//                 Tooltip::new()
//                     .trigger(Trigger::Item)
//                     .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow))
//             )
//             .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow).link(vec![AxisPointerLink::new().x_axis_index(1)]));
//         Ok(chart)
//     }
//
//     fn add_data_zoom(mut chart: Chart) -> Result<Chart, Error> {
//         chart = chart
//             .toolbox(
//                 Toolbox::new()
//                     .feature(
//                         Feature::new()
//                             // dynamic tool for click and drag zoom
//                             .data_zoom(ToolboxDataZoom::new()
//                                 .y_axis_index("all")
//                             )
//                             .restore(Restore::new())
//                             .save_as_image(SaveAsImage::new()
//                                 .type_(SaveAsImageType::Png)
//                                 .name("data_chart")
//                             )
//                     ))
//             // manual slider beneath graph
//             .data_zoom(
//                 DataZoom::new()
//                     .show(true)
//                     .realtime(true)
//                     .type_(DataZoomType::Slider)
//                     .start(0)
//                     .end(100))
//             // allows zoom via scrolling on trackpad (and maybe pinch on phones)
//             .data_zoom(
//                 DataZoom::new()
//                     .show(true)
//                     .type_(DataZoomType::Inside)
//                     .realtime(true)
//                     .start(0)
//                     .end(100))
//         ;
//         Ok(chart)
//
//         // .toolbox(
//         //         Toolbox::new().feature(
//         //             Feature::new()
//         //                 .data_zoom(ToolboxDataZoom::new()
//         //                     .y_axis_index("all")
//         //                 )
//         //                 .restore(Restore::new())
//         //                 .save_as_image(SaveAsImage::new()
//         //                     .type_(SaveAsImageType::Png)
//         //                     .name("data_chart")
//         //                 ),
//         //         ),
//         //     )
//     }
//
//     fn render_chart(chart: &Chart, title: String, image_width: u64, image_height: u64) -> Result<(), Error> {
//         let file_name = title.replace(" ", "-");
//         HtmlRenderer::new(title, image_width, image_height)
//             .save(&chart, "./graphics/".to_owned() + &*file_name + ".html")?;
//         Ok(())
//     }
// }
//
// impl Graph<DataRelationship> for Scatter {
//     fn graph(data: &DataRelationship) -> Result<(), Error> {
//         let mut chart = Scatter::create_chart(AxisType::Value, AxisType::Value)?;
//         let data_x_iter = data.data_x.data.clone().into_iter();
//         let mut data_y_iter = data.data_y.data.clone().into_iter();
//
//         let file_name = String::from(data.name.as_str().to_owned() + "_scatter");
//         let data_points: Vec<Vec<f64>> = data_points!(data_x_iter, data_y_iter);
//
//         chart = chart.series(Scatter::new().symbol_size(10).data(data_points.clone()).y_axis_index(0));
//         chart = chart.series(Scatter::new().symbol_size(10).data(data_points.clone()).y_axis_index(1));
//         chart = Self::add_data_zoom(chart)?;
//         info!("Generating and saving scatter plot as './graphics/{}'.html", file_name);
//         Scatter::render_chart(&chart, file_name, 1000, 800)?;
//
//         Ok(())
//     }
// }
//
// impl Graph<DataRelationship> for Line {
//     fn graph(data: &DataRelationship) -> Result<(), Error> {
//         let mut chart = Line::create_chart(AxisType::Value, AxisType::Value)?;
//         let data_x_iter = data.data_x.data.clone().into_iter();
//         let mut data_y_iter = data.fitted_values.clone().into_iter();
//
//         let file_name = String::from(data.name.as_str().to_owned() + "_line");
//
//         chart = chart.series(Line::new().symbol_size(10).data(data_points!(data_x_iter, data_y_iter)));
//         info!("Generating and saving line graph as './graphics/{}'.html", file_name);
//         Line::render_chart(&chart, file_name, 1000, 800)?;
//
//         Ok(())
//     }
// }
