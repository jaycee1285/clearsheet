pub struct CheatSheet {
    pub name: String,
    pub title: String,
    pub sections: Vec<Section>,
}

pub struct Section {
    pub header: String,
    pub rows: Vec<ShortcutRow>,
}

pub struct ShortcutRow {
    pub shortcut: String,
    pub description: String,
}

impl ShortcutRow {
    pub fn matches_query(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query = query.to_lowercase();
        self.shortcut.to_lowercase().contains(&query)
            || self.description.to_lowercase().contains(&query)
    }
}
