use crate::sheet::CheatSheet;

pub enum AppMode {
    Browser,
    Viewer,
}

pub struct App {
    pub mode: AppMode,
    pub sheets: Vec<CheatSheet>,
    pub selected_sheet: usize,
    pub scroll_offset: usize,
    pub filter_query: String,
    pub filtering: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(sheets: Vec<CheatSheet>, initial_sheet: Option<String>) -> Self {
        let (mode, selected_sheet) = match initial_sheet {
            Some(name) => {
                let idx = sheets
                    .iter()
                    .position(|s| s.name.eq_ignore_ascii_case(&name))
                    .unwrap_or(0);
                (AppMode::Viewer, idx)
            }
            None => (AppMode::Browser, 0),
        };

        App {
            mode,
            sheets,
            selected_sheet,
            scroll_offset: 0,
            filter_query: String::new(),
            filtering: false,
            should_quit: false,
        }
    }

    pub fn current_sheet(&self) -> Option<&CheatSheet> {
        self.sheets.get(self.selected_sheet)
    }
}
