mod basic_data_plotting;
mod define_chart_context;
mod draw_figure_components;
mod getting_started;

use basic_data_plotting::{
    draw_area_chart, draw_histogram_horz, draw_histogram_vert, draw_legend, draw_line_series,
    draw_multiple_series, draw_prime_dist, draw_scatter_plot, draw_time_series,
};
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
