use crate::screens::{Screen, ScreenWidget};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Default)]
pub struct App {
    screen: Screen,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.clear()?;
        terminal.hide_cursor()?;
        while !matches!(self.screen, Screen::Exit) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        self.screen.draw(frame);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        let event = event::read()?;
        if let Event::Key(key_event) = &event
            && key_event.kind == KeyEventKind::Press
            && key_event.modifiers == KeyModifiers::CONTROL
            && key_event.code == KeyCode::Char('c')
        {
            self.screen = Screen::Exit;
            return Ok(());
        }
        self.screen.handle_events(&event)?;
        Ok(())
    }
    pub fn login(username: String, host: String) -> io::Result<()> {
        let _ = (username, host);
        todo!()
    }
}
