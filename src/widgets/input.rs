use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};

#[derive(Default)]
pub struct Input {
    pub label: String,
    pub value: String,
    pub description: String,
    pub placeholder: String,
    is_focused: bool,
}

impl Input {
    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }
    pub fn is_focused(mut self, is_focused: bool) -> Self {
        self.is_focused = is_focused;
        self
    }
}

impl Widget for Input {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .title(self.label)
            .title_alignment(Alignment::Left)
            .title_bottom(self.description.dim())
            .border_style(if self.is_focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        Paragraph::new(if self.value.is_empty() {
            self.placeholder.bold().dim()
        } else {
            Span::raw(self.value)
        })
        .block(block)
        .render(area, buf);
    }
}
