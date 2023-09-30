mod getting_started;
mod define_chart_context;
mod draw_figure_components;

use getting_started::sin_wave;
use define_chart_context::{chart_builder, draw_series};
use draw_figure_components::{configure_mesh, configure_axes, configure_title};

fn main() {
    sin_wave();
    chart_builder();
    draw_series();
    configure_mesh();
    configure_axes();
    configure_title();
}