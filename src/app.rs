use crate::scanner::VenvInfo;
use std::path::PathBuf;

pub struct App {
    pub venvs: Vec<VenvInfo>,
    pub selected: usize,
    pub show_confirmation_dialog: bool,
    pub venv_to_delete_idx: Option<usize>,
}

impl App {
    pub fn new(venvs: Vec<VenvInfo>) -> Self {
        Self {
            venvs,
            selected: 0,
            show_confirmation_dialog: false,
            venv_to_delete_idx: None,
        }
    }

    pub fn start_deletion(&mut self) {
        if !self.venvs.is_empty() {
            self.show_confirmation_dialog = true;
            self.venv_to_delete_idx = Some(self.selected);
        }
    }

    pub fn confirm_deletion(&mut self) -> Option<PathBuf> {
        if self.show_confirmation_dialog {
            if let Some(idx) = self.venv_to_delete_idx {
                if idx < self.venvs.len() { // Ensure index is still valid
                    let removed_venv_path = self.venvs.remove(idx).path;
                    self.show_confirmation_dialog = false;
                    self.venv_to_delete_idx = None;

                    // Adjust selection
                    if self.venvs.is_empty() {
                        self.selected = 0;
                    } else if self.selected > idx {
                        // If selected was after the deleted item, move selection up by 1
                        self.selected = self.selected.saturating_sub(1);
                    } else if self.selected == idx && self.selected >= self.venvs.len() {
                        // If selected was the deleted item and now points beyond the end
                        self.selected = self.venvs.len() - 1;
                    }
                    // Ensure selected is within bounds
                    if self.selected >= self.venvs.len() && !self.venvs.is_empty() {
                        self.selected = self.venvs.len() - 1;
                    }
                    
                    return Some(removed_venv_path);
                }
            }
        }
        // If conditions not met, reset dialog state just in case
        self.show_confirmation_dialog = false;
        self.venv_to_delete_idx = None;
        None
    }

    pub fn cancel_deletion(&mut self) {
        self.show_confirmation_dialog = false;
        self.venv_to_delete_idx = None;
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.selected + 1 < self.venvs.len() {
            self.selected += 1;
        }
    }

    // pub fn selected_venv(&self) -> Option<&VenvInfo> {
    //     self.venvs.get(self.selected)
    // }
}
