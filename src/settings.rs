/// The tabulator type, can be either soft (spaces) or hard (tabs).
pub enum TabType {
    Soft,
    Hard,
}

pub struct Settings {
    pub tab_type: TabType,
    pub tab_width: usize,
}
