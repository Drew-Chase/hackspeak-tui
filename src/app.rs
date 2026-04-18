use crate::screens::{Screen, ScreenWidget};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::sync::OnceLock;

static _APP: OnceLock<App> = OnceLock::new();

#[derive(Default)]
pub struct App {
    screen: Screen,
}

impl App {
    fn get() -> &'static App {
        _APP.get_or_init(App::default)
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.clear()?;
        terminal.hide_cursor()?;
        while self.screen != Screen::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        self.screen.draw(frame);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if let (KeyModifiers::CONTROL, KeyCode::Char('c')) =
                    (key_event.modifiers, key_event.code)
                {
                    self.screen = Screen::Exit
                }
            }
            _ => {}
        }
        self.screen.handle_events()?;
        Ok(())
    }
    pub fn login(username: String, host: String)-> io::Result<()> {
        let mut app = App::get();
        todo!()
    }
}
