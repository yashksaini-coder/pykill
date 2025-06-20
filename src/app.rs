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
                    } else if self.selected >= idx && self.selected > 0 {
                        // If selected was at or after the deleted item, move selection up
                        // unless it was already the first item.
                        self.selected = self.selected.saturating_sub(1);
                    }
                    // Ensure selected is within bounds (it might be if the last item was selected and deleted)
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

    pub fn selected_venv(&self) -> Option<&VenvInfo> {
        self.venvs.get(self.selected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::VenvInfo; // Already in scope due to super::* but explicit for clarity
    use std::path::PathBuf;

    // Helper function to create dummy VenvInfo objects
    fn dummy_venv(name: &str) -> VenvInfo {
        VenvInfo {
            path: PathBuf::from(name),
            size: 100, // Dummy size
            last_modified: None, // Dummy last_modified
        }
    }

    #[test]
    fn test_app_new() {
        let venvs = vec![dummy_venv("venv1"), dummy_venv("venv2")];
        let app = App::new(venvs.clone());

        assert_eq!(app.selected, 0);
        assert_eq!(app.venvs.len(), 2);
        assert_eq!(app.venvs[0].path, PathBuf::from("venv1"));
        assert_eq!(app.venvs[1].path, PathBuf::from("venv2"));
        assert_eq!(app.show_confirmation_dialog, false);
        assert_eq!(app.venv_to_delete_idx, None);
    }

    #[test]
    fn test_app_navigation() {
        let venvs = vec![dummy_venv("v1"), dummy_venv("v2"), dummy_venv("v3")];
        let mut app = App::new(venvs);

        // Test next()
        assert_eq!(app.selected, 0);
        app.next();
        assert_eq!(app.selected, 1);
        app.next();
        assert_eq!(app.selected, 2);
        app.next(); // Try to go past the end
        assert_eq!(app.selected, 2);

        // Test previous()
        app.previous();
        assert_eq!(app.selected, 1);
        app.previous();
        assert_eq!(app.selected, 0);
        app.previous(); // Try to go before the start
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_app_navigation_empty_list() {
        let mut app = App::new(Vec::new());
        assert_eq!(app.selected, 0);
        app.next();
        assert_eq!(app.selected, 0);
        app.previous();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_app_navigation_single_item() {
        let mut app = App::new(vec![dummy_venv("v_single")]);
        assert_eq!(app.selected, 0);
        app.next();
        assert_eq!(app.selected, 0);
        app.previous();
        assert_eq!(app.selected, 0);
    }

    #[test]
    fn test_app_start_deletion() {
        let mut app = App::new(vec![dummy_venv("v1"), dummy_venv("v2")]);
        app.selected = 1; // Select the second item

        app.start_deletion();
        assert_eq!(app.show_confirmation_dialog, true);
        assert_eq!(app.venv_to_delete_idx, Some(1));
    }
    
    #[test]
    fn test_app_start_deletion_empty_list() {
        let mut app = App::new(vec![]);
        app.start_deletion();
        assert_eq!(app.show_confirmation_dialog, false); // Should not start deletion if list is empty
        assert_eq!(app.venv_to_delete_idx, None);
    }


    #[test]
    fn test_app_cancel_deletion() {
        let mut app = App::new(vec![dummy_venv("v1")]);
        app.start_deletion(); // Set state to show dialog

        app.cancel_deletion();
        assert_eq!(app.show_confirmation_dialog, false);
        assert_eq!(app.venv_to_delete_idx, None);
    }

    #[test]
    fn test_app_confirm_deletion_middle_item() {
        let venvs = vec![dummy_venv("v1"), dummy_venv("v2_to_delete"), dummy_venv("v3")];
        let mut app = App::new(venvs);
        app.selected = 1; // Select "v2_to_delete"

        app.start_deletion();
        let deleted_path = app.confirm_deletion();

        assert!(deleted_path.is_some());
        assert_eq!(deleted_path.unwrap(), PathBuf::from("v2_to_delete"));
        assert_eq!(app.show_confirmation_dialog, false);
        assert_eq!(app.venv_to_delete_idx, None);
        assert_eq!(app.venvs.len(), 2);
        assert_eq!(app.venvs[0].path, PathBuf::from("v1"));
        assert_eq!(app.venvs[1].path, PathBuf::from("v3"));
        assert_eq!(app.selected, 1); // Selected should now point to "v3" (new index 1)
    }

    #[test]
    fn test_app_confirm_deletion_first_item() {
        let venvs = vec![dummy_venv("v1_to_delete"), dummy_venv("v2"), dummy_venv("v3")];
        let mut app = App::new(venvs);
        app.selected = 0; // Select "v1_to_delete"

        app.start_deletion();
        let deleted_path = app.confirm_deletion();

        assert_eq!(deleted_path.unwrap(), PathBuf::from("v1_to_delete"));
        assert_eq!(app.venvs.len(), 2);
        assert_eq!(app.venvs[0].path, PathBuf::from("v2"));
        assert_eq!(app.venvs[1].path, PathBuf::from("v3"));
        assert_eq!(app.selected, 0); // Selected should remain 0, now pointing to "v2"
    }

    #[test]
    fn test_app_confirm_deletion_last_item() {
        let venvs = vec![dummy_venv("v1"), dummy_venv("v2"), dummy_venv("v3_to_delete")];
        let mut app = App::new(venvs);
        app.selected = 2; // Select "v3_to_delete"

        app.start_deletion();
        let deleted_path = app.confirm_deletion();

        assert_eq!(deleted_path.unwrap(), PathBuf::from("v3_to_delete"));
        assert_eq!(app.venvs.len(), 2);
        assert_eq!(app.venvs[0].path, PathBuf::from("v1"));
        assert_eq!(app.venvs[1].path, PathBuf::from("v2"));
        assert_eq!(app.selected, 1); // Selected should now point to "v2" (new last item)
    }
    
    #[test]
    fn test_app_confirm_deletion_selected_last_item_becomes_new_last() {
        let venvs = vec![dummy_venv("v1"), dummy_venv("v2_to_delete"), dummy_venv("v3_selected_then_deleted")];
        let mut app = App::new(venvs);
        app.selected = 2; // Select "v3_selected_then_deleted"
        
        app.start_deletion(); // venv_to_delete_idx is 2
        let deleted_path = app.confirm_deletion();

        assert_eq!(deleted_path.unwrap(), PathBuf::from("v3_selected_then_deleted"));
        assert_eq!(app.venvs.len(), 2);
        assert_eq!(app.selected, 1); // Selected should be last valid index (1)
    }


    #[test]
    fn test_app_confirm_deletion_only_item() {
        let venvs = vec![dummy_venv("v_only_to_delete")];
        let mut app = App::new(venvs);
        app.selected = 0;

        app.start_deletion();
        let deleted_path = app.confirm_deletion();

        assert_eq!(deleted_path.unwrap(), PathBuf::from("v_only_to_delete"));
        assert_eq!(app.venvs.len(), 0);
        assert_eq!(app.selected, 0); // Selected should be 0 as list is empty
    }
    
    #[test]
    fn test_app_confirm_deletion_without_start() {
        // Scenario: confirm_deletion is called when show_confirmation_dialog is false
        let mut app = App::new(vec![dummy_venv("v1")]);
        let deleted_path = app.confirm_deletion();
        assert!(deleted_path.is_none());
        assert_eq!(app.venvs.len(), 1); // No change
        assert_eq!(app.show_confirmation_dialog, false); // Should remain false
        assert_eq!(app.venv_to_delete_idx, None); // Should remain None
    }


    #[test]
    fn test_app_selected_venv() {
        let venv1 = dummy_venv("v1");
        let venv2 = dummy_venv("v2");
        let venvs = vec![venv1.clone(), venv2.clone()];
        let mut app = App::new(venvs);

        app.selected = 0;
        assert_eq!(app.selected_venv().unwrap().path, venv1.path);

        app.selected = 1;
        assert_eq!(app.selected_venv().unwrap().path, venv2.path);
    }

    #[test]
    fn test_app_selected_venv_empty_list() {
        let app = App::new(Vec::new());
        assert!(app.selected_venv().is_none());
    }
}
