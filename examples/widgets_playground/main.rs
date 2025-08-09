// use nih

extern crate nih_plug;

use nih_plug::prelude::*;

mod widgets_playground;
use widgets_playground::EguiUtilsExamplePlugin;

fn main() {
    nih_export_standalone::<EguiUtilsExamplePlugin>();
}
