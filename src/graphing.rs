// use std::fs::File;
// use std::path::Path;
// use plotters::prelude::*;
// use anyhow::{Result, Error};
// use crate::data_types::relationship::Relationship;
//
// pub fn graph_test(title: String, relationship: Relationship) -> Result<(), Error> {
//     let binding = String::from("./graphics/0.png");
//     let path = Path::new(&binding);
//     File::create(path)?;
//     let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
//     root.fill(&WHITE)?;
//     let mut chart = ChartBuilder::on(&root)
//         .caption(title, ("sans-serif", 50).into_font())
//         .margin(5)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
//
//     chart.configure_mesh().draw()?;
//
//     // quadratic function
//     // (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x))
//
//     chart
//         .draw_series(LineSeries::new(relationship.data_x.data.iter().map(|x|), &RED))?
//         .label("test")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
//
//     chart
//         .configure_series_labels()
//         .background_style(&WHITE.mix(0.8))
//         .border_style(&BLACK)
//         .draw()?;
//
//     root.present()?;
//
//     Ok(())
// }