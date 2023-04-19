#![allow(clippy::disallowed_methods)]

use renderz::App;

fn main() {
    let app = App::new().build().unwrap();
    app.run().unwrap();
}
