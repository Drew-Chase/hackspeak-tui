use crate::screens::ScreenWidget;
use crate::widgets::input::Input;
use rat_widget::button::ButtonState;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

#[derive(Default)]
pub struct ConnectScreen {
	username: String,
	host: String,
	login_button_state: ButtonState,
}

impl PartialEq for ConnectScreen {
	fn eq(&self, other: &Self) -> bool {
		self.username == other.username && self.host == other.host
	}
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
			.label(" Username ")
			.placeholder("  mr.pink")
			.description(" The username you want to use. ");
		let host_input = Input::default()
			.label(" Host ")
			.placeholder("  irc.freenode.net")
			.description(" The domain or ip and port you want to connect to. ");
		let button = rat_widget::button::Button::new("Login");

		let center = area.centered(Constraint::Percentage(50), Constraint::Length(9));
		let vertical = Layout::vertical((0..4).map(|_| Constraint::Length(4)));
		let rows = vertical.split(center);

		Paragraph::new(title).bold().render(rows[0], buf);
		username_input.render(rows[1], buf);
		host_input.render(rows[2], buf);
		button.render(rows[3], buf, &mut self.login_button_state);
	}
}
