use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScrapboxExport {
    pub pages: Vec<Page>,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    pub title: String,
    pub lines: Vec<Line>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Line {
    Object { text: String },
    String(String),
}

impl Line {
    pub fn text(&self) -> &str {
        match self {
            Line::Object { text } => text,
            Line::String(s) => s,
        }
    }
}
