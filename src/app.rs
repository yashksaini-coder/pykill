use crate::scanner::VenvInfo;

pub struct App {
    pub venvs: Vec<VenvInfo>,
    pub selected: usize,
}

impl App {
    pub fn new(venvs: Vec<VenvInfo>) -> Self {
        Self {
            venvs,
            selected: 0,
        }
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

    pub fn selected_venv(&self) -> Option<&VenvInfo> {
        self.venvs.get(self.selected)
    }
}
