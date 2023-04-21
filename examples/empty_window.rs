#![allow(clippy::disallowed_methods)]

use renderz::prelude::*;

fn main() {
    App::new()
        .is_resizable(true)
        .with_initial_window_size((800, 600))
        .with_min_window_size((800, 600))
        .with_background_color(Color::WHITE)
        .build()
        .unwrap()
        .run()
        .unwrap();
}
