use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Color},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use humansize::{format_size, DECIMAL};

// Helper function to create a centered rect for the dialog
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    // Main layout (rendered underneath the dialog if active)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content (list)
            Constraint::Length(3), // Help text
        ])
        .split(f.size());

    // List of virtual environments
    let list_block = Block::default()
        .title("Detected Virtual Environments")
        .borders(Borders::ALL);

    let items: Vec<ListItem> = app
        .venvs
        .iter()
        .enumerate()
        .map(|(i, venv)| {
            let size_formatted = format_size(venv.size, DECIMAL);
            let modified_str = match venv.last_modified {
                Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
                None => "N/A".to_string(),
            };
            let content_text = format!(
                "{} - {} - {}",
                venv.path.display(),
                size_formatted,
                modified_str
            );
            let content = Line::from(Span::raw(content_text));

            if i == app.selected {
                ListItem::new(content).style(Style::default().add_modifier(Modifier::REVERSED))
            } else {
                ListItem::new(content)
            }
        })
        .collect();

    let venv_list = List::new(items).block(list_block);
    f.render_widget(venv_list, main_chunks[0]);

    // General help text
    let help_block = Block::default().borders(Borders::ALL);
    let help_text_content = "Use ↑/↓ to navigate, 'd' to delete, 'q' to quit";
    let help_text = Paragraph::new(help_text_content).block(help_block);
    f.render_widget(help_text, main_chunks[1]);

    // Confirmation Dialog (Overlay)
    if app.show_confirmation_dialog {
        if let Some(idx) = app.venv_to_delete_idx {
            if let Some(venv_to_delete) = app.venvs.get(idx) {
                let dialog_block = Block::default()
                    .title("⚠ Confirm Deletion")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Red));

                let path_str = venv_to_delete.path.display().to_string();
                let confirmation_lines = vec![
                    Line::from(""),
                    Line::from(Span::styled("Are you sure you want to delete:", Style::default().fg(Color::White))),
                    Line::from(""),
                    Line::from(Span::styled(path_str, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Press ", Style::default().fg(Color::White)),
                        Span::styled("Y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                        Span::styled(" to confirm or ", Style::default().fg(Color::White)),
                        Span::styled("N", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                        Span::styled(" to cancel", Style::default().fg(Color::White)),
                    ]),
                    Line::from(""),
                ];
                
                let dialog_paragraph = Paragraph::new(confirmation_lines)
                    .block(dialog_block)
                    .alignment(ratatui::layout::Alignment::Center);

                let area = centered_rect(70, 30, f.size()); // Slightly larger dialog
                f.render_widget(Clear, area); // Clear the area before rendering
                f.render_widget(dialog_paragraph, area);
            }
        }
    }
}
