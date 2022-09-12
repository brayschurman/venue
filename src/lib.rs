#[macro_use]
extern crate vst;

use vst::plugin::{ Info, Plugin, Category };

#[derive(Default)]
struct Venue;

impl Plugin for Venue {
    fn get_info(&self) -> Info {
        Info {
            name: "Venue".to_string(),
            unique_id: 39829,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            ..Default::default()
        }
    }
}

plugin_main!(Venue);