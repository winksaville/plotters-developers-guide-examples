// https://plotters-rs.github.io/book/intro/getting_started.html
pub mod getting_started {
    use plotters::prelude::*;

    // https://plotters-rs.github.io/book/intro/getting_started.html#step-2---add-plotting-code
    pub fn sin_wave() {
        let root_drawing_area =
            BitMapBackend::new("images/0.1.png", (1024, 768)).into_drawing_area();

        root_drawing_area.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root_drawing_area)
            .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
                &RED,
            ))
            .unwrap();
    }
}

// https://plotters-rs.github.io/book/basic/overview.html
pub mod basic_plotting {

    // https://plotters-rs.github.io/book/chart_context
    pub mod define_chart_context {
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
    }
}

fn main() {
    getting_started::sin_wave();
    basic_plotting::define_chart_context::chart_builder();
    basic_plotting::define_chart_context::draw_series();
}
