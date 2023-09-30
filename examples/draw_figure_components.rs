use plotters::prelude::*;

// https://plotters-rs.github.io/book/basic/chart_components.html#mesh
pub fn configure_mesh() {
    let root_drawing_area = BitMapBackend::new("images/2.2.png", (600, 400)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
}

// https://plotters-rs.github.io/book/basic/chart_components.html#axes
pub fn configure_axes() {
    let root_drawing_area = BitMapBackend::new("images/2.3.png", (600, 400)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        // enables Y axis, the size is 40 px
        .set_label_area_size(LabelAreaPosition::Left, 40)
        // enable X axis, the size is 40 px
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
}

// https://plotters-rs.github.io/book/basic/chart_components.html#title
pub fn configure_title() {
    let root_drawing_area = BitMapBackend::new("images/2.4.png", (600, 400)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Figure Sample", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();
}

#[allow(dead_code)]
fn main() {
    configure_mesh();
    configure_axes();
    configure_title();
}
