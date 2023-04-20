#![allow(clippy::disallowed_methods)]

use renderz::App;

fn main() {
    let app = App::new()
        .is_resizable(true)
        .with_initial_size((800, 600))
        .build()
        .unwrap();
    app.run().unwrap();
}
