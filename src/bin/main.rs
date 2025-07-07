use anyhow::Result;
use taskrs::{
    app::App,
    ui::{daily::DailyView, monthly::MonthlyView},
};

fn main() -> Result<()> {
    let mut term = ratatui::init();

    let mut daily_view = DailyView::new();
    let mut monthly_view = MonthlyView::new();
    let mut app = App::new(&mut monthly_view, &mut daily_view);

    let res = app.run(&mut term);

    ratatui::restore();

    res
}
