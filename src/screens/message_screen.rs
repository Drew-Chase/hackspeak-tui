use crate::screens::ScreenWidget;
use crossterm::event::Event;
use ratatui::prelude::*;

#[derive(Default)]
pub struct MessageScreen {
    username: String,
    host: String,
}

impl ScreenWidget for MessageScreen {
    fn draw(&mut self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self, event: &Event) -> std::io::Result<()> {
        let _ = event;
        todo!()
    }
}

impl Widget for MessageScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
