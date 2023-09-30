use plotters::prelude::*;

pub fn draw_3d_env() {
    let root = BitMapBackend::new("images/3d-env.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Empty 3D Figure", ("sans-serif", 40))
        .build_cartesian_3d(0.0..1.0, 0.0..1.0, 0.0..1.0)
        .unwrap();
    chart.configure_axes().draw().unwrap();
}

pub fn draw_3d_line() {
    let root = BitMapBackend::new("images/3d-line.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Line", ("sans-serif", 40))
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
        .unwrap();
    chart.configure_axes().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            (-100..100)
                .map(|y| y as f64 / 100.0)
                .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &RED,
        ))
        .unwrap();
}

pub fn draw_3d_bar() {
    let root = BitMapBackend::new("images/3d-bar.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Bar", ("sans-serif", 40))
        .build_cartesian_3d(0i32..10, -10i32..10, 0i32..10)
        .unwrap();
    chart.configure_axes().draw().unwrap();

    chart
        .draw_series(
            (0i32..10i32)
                .map(|x| std::iter::repeat(x).zip(0i32..10))
                .flatten()
                .map(|(x, z)| {
                    Cubiod::new([(x, 0, z), (x + 1, x - z, z + 1)], BLUE.filled(), &BLACK)
                }),
        )
        .unwrap();
}

// https://plotters-rs.github.io/book/basic/draw_3d_plots.html#3d-surface
pub fn draw_3d_surface() {
    let root = BitMapBackend::new("images/3d-surface.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Surface", ("sans-serif", 40))
        .build_cartesian_3d(-3.0..3.0, -3.0..3.0, -3.0..3.0)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    let mut data = vec![];

    for x in (-25..25).map(|v| v as f64 / 10.0) {
        let mut row = vec![];
        for z in (-25..25).map(|v| v as f64 / 10.0) {
            row.push((x, (x * x + z * z).cos(), z));
        }
        data.push(row);
    }

    chart
        .draw_series(
            (0..49)
                .map(|x| std::iter::repeat(x).zip(0..49))
                .flatten()
                .map(|(x, z)| {
                    Polygon::new(
                        vec![
                            data[x][z],
                            data[x + 1][z],
                            data[x + 1][z + 1],
                            data[x][z + 1],
                        ],
                        &BLUE.mix(0.3),
                    )
                }),
        )
        .unwrap();
}

// https://plotters-rs.github.io/book/basic/draw_3d_plots.html#drawing-with-surface-series
pub fn draw_3d_surface_series() {
    let root = BitMapBackend::new("images/3d-surface-series.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Surface", ("sans-serif", 40))
        .build_cartesian_3d(-3.0..3.0, -3.0..3.0, -3.0..3.0)
        .unwrap();

    chart.configure_axes().draw().unwrap();

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-25..25).map(|v| v as f64 / 10.0),
                (-25..25).map(|v| v as f64 / 10.0),
                |x: f64, z: f64| (x * x + z * z).cos(),
            )
            .style(&BLUE.mix(0.2)),
        )
        .unwrap();
}

// https://plotters-rs.github.io/book/basic/draw_3d_plots.html#customize-perspective-matrix
pub fn draw_3d_matrix() {
    let root = BitMapBackend::new("images/3d-matrix.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
        .unwrap();

    // You need configure the projection matrix before start drawing anything
    chart.with_projection(|mut pb| {
        pb.pitch = 1.2;
        pb.yaw = 0.5;
        pb.scale = 0.7;
        pb.into_matrix()
    });
    chart.configure_axes().draw().unwrap();
    chart
        .draw_series(LineSeries::new(
            (-100..100)
                .map(|y| y as f64 / 100.0)
                .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &RED,
        ))
        .unwrap();
}

#[allow(dead_code)]
fn main() {
    draw_3d_env();
    draw_3d_line();
    draw_3d_bar();
    draw_3d_surface();
    draw_3d_surface_series();
    draw_3d_matrix();
}
