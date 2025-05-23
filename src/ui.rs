use crate::app::App;

pub fn draw_ui(app: &App) {
    println!("Found {} virtual environments:\n", app.venvs.len());

    for (i, venv) in app.venvs.iter().enumerate() {
        let marker = if i == app.selected { ">" } else { " " };
        let size_mb = venv.size / 1024 / 1024;

        println!(
            "{} [{} MB] {}",
            marker,
            size_mb,
            venv.path.display()
        );
    }
}
