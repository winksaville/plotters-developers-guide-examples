mod getting_started;
mod define_chart_context;

use getting_started::sin_wave;
use define_chart_context::{chart_builder, draw_series};


fn main() {
    sin_wave();
    chart_builder();
    draw_series();
}