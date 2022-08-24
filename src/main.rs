mod app;
mod args;
mod config;
mod dir;
mod fs;
mod note;
mod vault;

use app::App;

fn main() {
    let mut app: App = App::new();
    app.load_vault();
    app.handle_args()
}
