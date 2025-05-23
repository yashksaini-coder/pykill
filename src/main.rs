mod scanner;
mod utils;
mod app;
mod ui;

use std::env;
use app::App;
use scanner::scan_for_venvs;
use ui::draw_ui;
use utils::delete_venv;

fn main() {
    let cwd = env::current_dir().unwrap();
    let venvs = scan_for_venvs(&cwd);

    let mut app = App::new(venvs);

    draw_ui(&app);

    println!("\n-- Moving down --");
    app.next();
    draw_ui(&app);

    println!("\n-- Moving up --");
    app.previous();
    draw_ui(&app);

    if let Some(sel) = app.selected_venv() {
        println!("\nCurrently selected venv: {}", sel.path.display());
        println!("[Dry-run] Would delete: {}", sel.path.display());
        // Uncomment to actually delete:
        delete_venv(&sel.path).unwrap();
    }
}
