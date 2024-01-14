use std::f32::consts::{E, PI};
use plotters::prelude::*;

fn normal_dist(x: f32, mu: f32, sigma: f32) -> f32 {
    let y = 1.0 / (sigma * (2.0 * PI).sqrt()) * 
    E.powf(-1.0 * ((x - mu) / sigma).powi(2) /  (2.0 * sigma.powi(2)));
    y
}
fn main() {
    let plot_range: Vec<i32> = (-50..=49).collect();
    let mut normal_vals = [0.0; 100];
    plot_range.iter().enumerate().for_each(|(i, x)| {
        normal_vals[i] = normal_dist(*x as f32, 0_f32, 1.0);
    });
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    let _ = root.fill(&WHITE);

    let mut chart = ChartBuilder::on(&root)
        .caption("Normal Distribution", ("Arial", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-50f32..50f32, 0f32..1f32).unwrap();


    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(
        plot_range.iter().map(|x| *x as f32).zip(normal_vals.iter().cloned()),
        &RED,
    )).unwrap();

    root.present().unwrap();
}
