use crate::app::App;
use crate::screens::ScreenWidget;
use crossterm::event::Event;
use rat_widget::button::{Button, ButtonState};
use rat_widget::event::{ButtonOutcome, HandleEvent, Regular, TextOutcome};
use rat_widget::focus::{Focus, FocusBuilder};
use rat_widget::text::HasScreenCursor;
use rat_widget::text_input::{TextInput, TextInputState};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub struct ConnectScreen {
    username: TextInputState,
    host: TextInputState,
    login_button: ButtonState,
    create_button: ButtonState,
}

impl Default for ConnectScreen {
    fn default() -> Self {
        Self {
            username: TextInputState::new_focused(),
            host: TextInputState::new(),
            login_button: ButtonState::default(),
            create_button: ButtonState::default(),
        }
    }
}

impl ConnectScreen {
    fn focus(&self) -> Focus {
        let mut fb = FocusBuilder::new(None);
        fb.widget(&self.username);
        fb.widget(&self.host);
        fb.widget(&self.login_button);
        fb.widget(&self.create_button);
        fb.build()
    }
}

impl ScreenWidget for ConnectScreen {
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let title = Line::from("Login:".bold());

        let username_block = Block::default()
            .title(" Username ")
            .title_bottom(" The username you want to use. ".dim())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);
        let username_input = TextInput::default().block(username_block);

        let host_block = Block::default()
            .title(" Host ")
            .title_bottom(" The domain or ip and port you want to connect to. ".dim())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);
        let host_input = TextInput::default().block(host_block);

        let connect_button = Button::new("Connect").block(Block::bordered());
        let create_button = Button::new("Create Server").block(Block::bordered());

        let center = area.centered(Constraint::Percentage(50), Constraint::Length(13));
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]);
        let rows = vertical.split(center);

        let button_row =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let buttons = button_row.split(rows[3]);

        frame.render_widget(Paragraph::new(title).bold(), rows[0]);
        frame.render_stateful_widget(username_input, rows[1], &mut self.username);
        frame.render_stateful_widget(host_input, rows[2], &mut self.host);
        frame.render_stateful_widget(connect_button, buttons[0], &mut self.login_button);
        frame.render_stateful_widget(create_button, buttons[1], &mut self.create_button);

        if let Some(pos) = self
            .username
            .screen_cursor()
            .or_else(|| self.host.screen_cursor())
        {
            frame.set_cursor_position(pos);
        }
    }

    fn handle_events(&mut self, event: &Event) -> std::io::Result<()> {
        let mut focus = self.focus();
        let _ = focus.handle(event, Regular);

        let _: TextOutcome = self.username.handle(event, Regular);
        let _: TextOutcome = self.host.handle(event, Regular);

        if let ButtonOutcome::Pressed = self.login_button.handle(event, Regular) {
            let username = self.username.text().to_string();
            let host = self.host.text().to_string();
            App::login(username.clone(), host)?;
        }

        if let ButtonOutcome::Pressed = self.create_button.handle(event, Regular) {
            let username = self.username.text().to_string();
            let host = self.host.text().to_string();
            App::create()?;
            App::login(username, host)?;
        }
        Ok(())
    }
}
