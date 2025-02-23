use color_eyre::Result;

mod app;
mod draw;
mod input;
use crate::app::App;

// driver function for the whole app
fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
