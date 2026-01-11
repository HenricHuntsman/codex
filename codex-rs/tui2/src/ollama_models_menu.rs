use ratatui::text::Line;
use ratatui::widgets::{Paragraph, WidgetRef};
use ratatui::style::{Color, Stylize};
use ratatui::layout::Rect;
use ratatui::buffer::Buffer;

/// Widget for managing Ollama models: list, pull, switch.
pub struct OllamaModelsMenu {
    pub models: Vec<String>,
    pub selected: usize,
    pub status: String,
}

impl OllamaModelsMenu {
    pub fn new(models: Vec<String>, status: String) -> Self {
        Self {
            models,
            selected: 0,
            status,
        }
    }

    pub fn next(&mut self) {
        if !self.models.is_empty() {
            self.selected = (self.selected + 1) % self.models.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.models.is_empty() {
            if self.selected == 0 {
                self.selected = self.models.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn selected_model(&self) -> Option<&String> {
        self.models.get(self.selected)
    }
}

impl WidgetRef for &OllamaModelsMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<Line> = vec![
            Line::from("Ollama Models Menu".bold().cyan()),
            Line::from(""),
        ];
        for (i, model) in self.models.iter().enumerate() {
            if i == self.selected {
                lines.push(Line::from(format!("> {model}").bold().green()));
            } else {
                lines.push(Line::from(format!("  {model}")));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(self.status.clone().dim()));
        let para = Paragraph::new(lines).wrap(ratatui::widgets::Wrap { trim: false });
        para.render(area, buf);
    }
}
