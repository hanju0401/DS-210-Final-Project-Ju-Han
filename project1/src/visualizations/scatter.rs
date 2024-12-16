use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;

pub fn scatter_plot_points_vs_win_percentage(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("points_vs_win_percentage.png", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Points vs. Win Percentage", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..40.0, 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    let points: Vec<f64> = df.column("team_avg_points")?
        .f64()?
        .into_iter()
        .flatten()
        .collect();

    let win_percentages: Vec<f64> = df.column("win_percentage")?
        .f64()?
        .into_iter()
        .flatten()
        .collect();

    chart.draw_series(
        points.iter().zip(win_percentages.iter()).map(|(&x, &y)| {
            Circle::new((x, y), 5, Into::<ShapeStyle>::into(&RED).filled())
        }),
    )?;

    println!("Scatter plot of Points vs. Win Percentage saved to points_vs_win_percentage.png");
    Ok(())
}
