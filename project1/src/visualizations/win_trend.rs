use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;

pub fn visualize_win_percentage(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("win_percentage_trend.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Win Percentage Trend", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..df.height() as i32, 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    let win_percentages: Vec<f64> = df.column("win_percentage")?
        .f64()?
        .into_iter()
        .flatten()
        .collect();

    chart.draw_series(LineSeries::new(
        (0..win_percentages.len() as i32).zip(win_percentages.iter().cloned()),
        &BLUE,
    ))?;

    println!("Win Percentage Trend visualization saved to win_percentage_trend.png");
    Ok(())
}
