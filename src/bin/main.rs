use anyhow::Result;
use taskrs::app::App;

fn main() -> Result<()> {
    let mut term = ratatui::init();
    let mut app = App::new();

    let res = app.run(&mut term);

    ratatui::restore();

    res
}
