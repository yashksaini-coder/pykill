use crate::app::App;

pub fn draw_ui(app: &App) {
    println!("Found {} virtual environments:\n", app.venvs.len());

    for (i, venv) in app.venvs.iter().enumerate() {
        let marker = if i == app.selected { ">" } else { " " };
        let size_mb = venv.size / 1024 / 1024;
        let modified = match venv.last_modified {
            Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
            None => "N/A".to_string(),
        };

        println!(
            "{} [{} MB] [{}] {}",
            marker,
            size_mb,
            modified,
            venv.path.display()
        );
    }
}
