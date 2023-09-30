use plotters::prelude::*;

const PI: f64 = std::f64::consts::PI;

// https://plotters-rs.github.io/book/intro/getting_started.html#step-2---add-plotting-code
pub fn sin_wave() {
    let root_drawing_area =
        BitMapBackend::new("images/0.1.png", (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-PI..PI, -1.2..1.2)
        .unwrap();

    chart.draw_series(LineSeries::new(
            (((-PI as f64 * 100.0) as i32)..(PI* 100.0) as i32).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
            &RED,
    )).unwrap();
}

#[allow(dead_code)]
fn main() {
    sin_wave();
}
