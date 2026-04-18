use crate::screens::connect_screen::ConnectScreen;
use ratatui::Frame;
use crate::screens::message_screen::MessageScreen;

pub mod connect_screen;
mod message_screen;

pub trait ScreenWidget {
    fn draw(&self, frame: &mut Frame);
    fn handle_events(&mut self) -> std::io::Result<()>;
}

#[derive(PartialEq, Eq)]
pub enum Screen {
    ConnectScreen(ConnectScreen),
    MessageScreen(MessageScreen),
    Exit,
}

impl Default for Screen {
    fn default() -> Self {
        Self::ConnectScreen(ConnectScreen::default())
    }
}

impl ScreenWidget for Screen {

    fn draw(&self, frame: &mut Frame) {
        match self {
            Screen::ConnectScreen(screen) => screen.draw(frame),
            Screen::MessageScreen(screen) => screen.draw(frame),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match self {
            Screen::ConnectScreen(screen) => screen.handle_events(),
            Screen::MessageScreen(screen) => screen.handle_events(),
            _ => Ok(()),
        }
    }
}
