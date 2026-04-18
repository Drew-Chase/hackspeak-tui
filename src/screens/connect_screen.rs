use crate::screens::ScreenWidget;
use crate::widgets::input::Input;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

#[derive(Default, PartialEq, Eq)]
pub struct ConnectScreen {
    username: String,
    host: String,
}
impl ScreenWidget for ConnectScreen {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Widget for &ConnectScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Login:".bold());
        let username_input = Input::default()
            .label("Username")
            .placeholder("mr.pink")
            .description("The username you want to use.");
        let host_input = Input::default()
            .label("Host")
            .placeholder("irc.freenode.net")
            .description("The domain or ip and port you want to connect to.");

        Paragraph::new(title).centered().render(area, buf);
        username_input.render(area, buf);
        host_input.render(area, buf);
    }
}
