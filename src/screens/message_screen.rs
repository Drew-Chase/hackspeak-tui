use crate::screens::ScreenWidget;
use ratatui::prelude::*;

#[derive(Default, PartialEq, Eq)]
pub struct MessageScreen {
    username: String,
    host: String,
}

impl ScreenWidget for MessageScreen {
    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
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
