mod visualizations;

use visualizations::win_trend::visualize_win_percentage;
use visualizations::scatter::scatter_plot_points_vs_win_percentage;
use polars::prelude::*;
use plotters::prelude::*;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array2, Array1};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    // Step 1: Load the dataset
    let file_path = "expanded_team_performance_nba_data.csv";
    let file = File::open(file_path)?;
    let df = CsvReader::new(file).finish()?;

    println!("Loaded dataset with shape: {:?}", df.shape());

    // Ensure dataset has at least 1000 rows
    assert!(df.height() >= 1000, "Dataset must contain at least 1000 rows.");

    // Step 2: Data preprocessing for regression
    let feature_columns = vec![
        DatasetColumns::Points.column_name(),
        DatasetColumns::Assists.column_name(),
        DatasetColumns::Rebounds.column_name(),
    ];
    let target_column = DatasetColumns::WinPercentage.column_name();

    let features: Vec<f64> = feature_columns
        .iter()
        .flat_map(|col| {
            df.column(col)
                .unwrap()
                .f64()
                .unwrap()
                .iter()
                .map(|x| x.unwrap_or(0.0))
        })
        .collect();

    let targets: Vec<f64> = df.column(target_column)
        .unwrap()
        .f64()
        .unwrap()
        .iter()
        .map(|x| x.unwrap_or(0.0))
        .collect();

    let feature_matrix = Array2::from_shape_vec((df.height(), feature_columns.len()), features)?;
    let target_vector = Array1::from(targets);

    // Step 3: Visualizations
    visualize_win_percentage(&df)?;
    scatter_plot_points_vs_win_percentage(&df)?;

    // Step 4: Run regression analysis
    multilinear_regression(&feature_matrix, &target_vector)?;

    Ok(())
}

#[derive(Debug)]
enum DatasetColumns {
    Points,
    Assists,
    Rebounds,
    WinPercentage,
}

impl DatasetColumns {
    fn column_name(&self) -> &str {
        match self {
            DatasetColumns::Points => "team_avg_points",
            DatasetColumns::Assists => "team_avg_assists",
            DatasetColumns::Rebounds => "team_avg_rebounds",
            DatasetColumns::WinPercentage => "win_percentage",
        }
    }
}

fn multilinear_regression(features: &Array2<f64>, targets: &Array1<f64>) -> Result<(), Box<dyn Error>> {
    let dataset = DatasetBase::new(features.view(), targets.view());

    // Train the regression model
    let model = LinearRegression::default().fit(&dataset)?;

    // Predict outcomes
    let predictions = model.predict(dataset.records());

    // Evaluate the model
    let mse = predictions
        .iter()
        .zip(targets.iter())
        .map(|(y_pred, y_true)| (y_pred - y_true).powi(2))
        .sum::<f64>()
        / targets.len() as f64;

    println!("Multilinear Regression Predictions: {:?}", predictions);
    println!("Mean Squared Error: {:.4}", mse);
    Ok(())
}
