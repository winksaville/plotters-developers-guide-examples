use plotters::prelude::*;

// https://plotters-rs.github.io/book/basic/chart_context.html#create-a-chart-context-from-a-drwaing-area
pub fn chart_builder() {
    let drawing_area =
        BitMapBackend::new("images/2.0.png", (1024, 768)).into_drawing_area();

    let _chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();
}

// https://plotters-rs.github.io/book/basic/chart_context.html#draw-series-on-to-the-chart-context
pub fn draw_series() {
    let drawing_area = BitMapBackend::new("images/2.1.png", (600, 400)).into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart
        .draw_series(LineSeries::new((0..100).map(|x| (x, 100 - x)), &BLACK))
        .unwrap();
}

#[allow(dead_code)]
fn main() {
    chart_builder();
    draw_series();
}
