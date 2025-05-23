mod scanner;
mod utils;
mod app;
mod ui;

use std::env;
use app::App;
use scanner::scan_for_venvs;
use ui::draw_ui;

fn main() {
    let cwd = env::current_dir().unwrap();
    let venvs = scan_for_venvs(&cwd);

    let mut app = App::new(venvs);

    // Simulate some navigation
    draw_ui(&app);
    println!("\n-- Moving down --");
    app.next();
    draw_ui(&app);
    println!("\n-- Moving up --");
    app.previous();
    draw_ui(&app);
}
