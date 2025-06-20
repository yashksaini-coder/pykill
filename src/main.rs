mod scanner;
mod utils;
mod app;
mod ui;

use std::path::PathBuf;
use app::App;
use scanner::scan_for_venvs;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::stdout;
use ui::draw_ui;
use utils::delete_venv;
use clap::Parser;
use humansize::{format_size, DECIMAL};

/// A simple TUI to find and delete Python virtual environments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Path to scan for virtual environments
    #[arg(default_value_os_t = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")))]
    scan_path: PathBuf,

    /// Run without the TUI, just print found venvs
    #[arg(long, short = 'n')]
    no_tui: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let venvs = scan_for_venvs(&args.scan_path);

    if args.no_tui {
        if venvs.is_empty() {
            println!("No virtual environments found at '{}'.", args.scan_path.display());
        } else {
            println!("Found {} virtual environments at '{}':\n", venvs.len(), args.scan_path.display());
            for venv in venvs {
                let size_formatted = format_size(venv.size, DECIMAL);
                let modified_str = venv
                    .last_modified
                    .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_else(|| "N/A".to_string());
                println!(
                    "  Path: {}\n    Size: {}\n    Last Modified: {}\n",
                    venv.path.display(),
                    size_formatted,
                    modified_str
                );
            }
        }
        return Ok(());
    }

    // Proceed with TUI if --no-tui is not set
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new(venvs);

    loop {
        terminal.draw(|f| {
            draw_ui(f, &app);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if app.show_confirmation_dialog {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            if let Some(path_to_delete) = app.confirm_deletion() {
                                if let Err(e) = delete_venv(&path_to_delete) {
                                    eprintln!("Error deleting venv {}: {}", path_to_delete.display(), e);
                                }
                            }
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            app.cancel_deletion();
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up => app.previous(),
                        KeyCode::Down => app.next(),
                        KeyCode::Char('d') | KeyCode::Char('D') => {
                            if !app.venvs.is_empty() {
                                app.start_deletion();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
