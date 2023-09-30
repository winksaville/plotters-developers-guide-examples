use plotters::prelude::*;

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#line-series
pub fn draw_line_series() {
    let root_area = BitMapBackend::new("images/2.5.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Line Plot Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..10, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new((-10..=10).map(|x| (x, x * x)), &GREEN))
        .unwrap();
}

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#scatter-plot
pub fn draw_scatter_plot() {
    let root_area = BitMapBackend::new("images/2.6.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-10..50, -10..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        DATA1
            .iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap();

    ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();
}

#[rustfmt::skip]
const DATA1: [(i32, i32); 30] =  [(-3, 1), (-2, 3), (4, 2), (3, 0), (6, -5), (3, 11), (6, 0), (2, 14), (3, 9), (14, 7), (8, 11), (10, 16), (7, 15), (13, 8), (17, 14), (13, 17), (19, 11), (18, 8), (15, 8), (23, 23), (15, 20), (22, 23), (22, 21), (21, 30), (19, 28), (22, 23), (30, 23), (26, 35), (33, 19), (26, 19)];
#[rustfmt::skip]
const DATA2: [(i32, i32); 30] = [(1, 22), (0, 22), (1, 20), (2, 24), (4, 26), (6, 24), (5, 27), (6, 27), (7, 27), (8, 30), (10, 30), (10, 33), (12, 34), (13, 31), (15, 35), (14, 33), (17, 36), (16, 35), (17, 39), (19, 38), (21, 38), (22, 39), (23, 43), (24, 44), (24, 46), (26, 47), (27, 48), (26, 49), (28, 47), (28, 50)];

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#area-chart
pub fn draw_area_chart() {
    let root_area = BitMapBackend::new("images/2.7.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..10, 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 29, 0, 21];

    ctx.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter().map(|x| *x)), // The data iter
            0,                                  // Baseline
            &RED.mix(0.2),                      // Make the series opac
        )
        .border_style(&RED), // Make a brighter border
    )
    .unwrap();
}

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#histogram
pub fn draw_histogram_vert() {
    let root_area = BitMapBackend::new("images/2.8.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("sans-serif", 40))
        .build_cartesian_2d((0..10).into_segmented(), 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();
}

pub fn draw_histogram_horz() {
    let root_area = BitMapBackend::new("images/2.9.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..50, (0..10).into_segmented())
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21, 5];

    ctx.draw_series((0..).zip(data.iter()).map(|(y, x)| {
        let mut bar = Rectangle::new(
            [
                (0, SegmentValue::Exact(y)),
                (*x, SegmentValue::Exact(y + 1)),
            ],
            GREEN.filled(),
        );
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
    .unwrap();
}

fn is_prime(n: i32) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#visualize-distribution
pub fn draw_prime_dist() {
    let root_area = BitMapBackend::new("images/2.13.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Prime Distribution", ("sans-serif", 40))
        .build_cartesian_2d([true, false].into_segmented(), 0..50)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let prim: Vec<_> = (2..50).map(is_prime).collect();

    ctx.draw_series(
        Histogram::vertical(&ctx)
            .margin(100)
            .data(prim.iter().map(|x| (x, 1))),
    )
    .unwrap();
}

//https://plotters-rs.github.io/book/basic/basic_data_plotting.html#time-series-chart
use chrono::{TimeZone, Utc};

pub fn draw_time_series() {
    let root_area = BitMapBackend::new("images/2.11.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let start_date_time = Utc.with_ymd_and_hms(2019, 10, 1, 0, 0, 0).unwrap();
    let end_date_time = Utc.with_ymd_and_hms(2019, 10, 18, 0, 0, 0).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("MSFT daily close price", ("sans-serif", 40))
        .build_cartesian_2d(
            start_date_time.date_naive()..end_date_time.date_naive(),
            130.0..145.0,
        )
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(LineSeries::new(
        (0..).zip(DATA.iter()).map(|(idx, price)| {
            let day = (idx / 5) * 7 + idx % 5 + 1;
            let date = Utc
                .with_ymd_and_hms(2019, 10, day, 0, 0, 0)
                .unwrap()
                .date_naive();
            (date, *price)
        }),
        &BLUE,
    ))
    .unwrap();
}

#[rustfmt::skip]
const DATA: [f64; 14] = [ 137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.10, 138.24, 135.67, 137.12, 138.12];

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#multiple-data-series
pub fn draw_multiple_series() {
    let root_area = BitMapBackend::new("images/2.10.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Histo + Line", ("sans-serif", 40))
        .build_cartesian_2d(0..10, 0..80)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21];

    // Draw the histogram
    ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
        let mut bar = Rectangle::new([(x, 0), (x + 1, *y)], GREEN.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();

    // Draw the line series
    ctx.draw_series(LineSeries::new(
        (0..).zip(data.iter()).map(|(x, y)| (x, *y + 30)),
        &BLUE,
    ))
    .unwrap();
}

// https://plotters-rs.github.io/book/basic/basic_data_plotting.html#legend
pub fn draw_legend() {
    let root_area = BitMapBackend::new("images/2.12.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Legend", ("sans-serif", 40))
        .build_cartesian_2d(-4.0..4.0, -1.2..1.2)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let x_kps: Vec<_> = (-80..80).map(|x| x as f64 / 20.0).collect();
    ctx.draw_series(LineSeries::new(x_kps.iter().map(|x| (*x, x.sin())), &RED))
        .unwrap()
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(LineSeries::new(x_kps.iter().map(|x| (*x, x.cos())), &BLUE))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}

#[allow(dead_code)]
fn main() {
    draw_line_series();
    draw_scatter_plot();
    draw_area_chart();
    draw_histogram_vert();
    draw_histogram_horz();
    draw_prime_dist();
    draw_time_series();
    draw_multiple_series();
    draw_legend();
}
