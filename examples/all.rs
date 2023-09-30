mod define_chart_context;
mod draw_figure_components;
mod getting_started;

use define_chart_context::{chart_builder, draw_series};
use draw_figure_components::{configure_axes, configure_mesh, configure_title};
use getting_started::sin_wave;

fn main() {
    sin_wave();
    chart_builder();
    draw_series();
    configure_mesh();
    configure_axes();
    configure_title();
}
