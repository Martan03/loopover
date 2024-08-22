use app::App;
use termint::geometry::Coords;

mod app;
mod board;
mod error;

fn main() {
    let mut app = App::new(Coords::new(3, 3));
    _ = app.run();
}
