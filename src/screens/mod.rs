use crate::screens::connect_screen::ConnectScreen;
use crate::screens::message_screen::MessageScreen;
use crossterm::event::Event;
use ratatui::Frame;

pub mod connect_screen;
mod message_screen;

pub trait ScreenWidget {
    fn draw(&mut self, frame: &mut Frame);
    fn handle_events(&mut self, event: &Event) -> std::io::Result<()>;
}

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
    fn draw(&mut self, frame: &mut Frame) {
        match self {
            Screen::ConnectScreen(screen) => screen.draw(frame),
            Screen::MessageScreen(screen) => screen.draw(frame),
            Screen::Exit => {}
        }
    }

    fn handle_events(&mut self, event: &Event) -> std::io::Result<()> {
        match self {
            Screen::ConnectScreen(screen) => screen.handle_events(event),
            Screen::MessageScreen(screen) => screen.handle_events(event),
            Screen::Exit => Ok(()),
        }
    }
}
