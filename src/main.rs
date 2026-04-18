mod app;
mod screens;
mod widgets;

use crate::app::App;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}
