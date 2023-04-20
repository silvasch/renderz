#![allow(clippy::disallowed_methods)]

use renderz::{App, Color};

fn main() {
    let app = App::new()
        .is_resizable(true)
        .with_initial_size((800, 600))
        .with_background_color(Color::from_rgba(0, 255, 255, 0.5))
        .build()
        .unwrap();
    app.run().unwrap();
}
