use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

// renders main 3/4 of w and journal 1/4 of w
pub fn render_layout<W: Widget, JW: Widget>(
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
    main: W,
    j: JW,
) {
    main.render(
        Rect::new(area.x, area.y, area.width * 3 / 4, area.height),
        buf,
    );

    render_vertical_line(
        Rect::new(area.x + area.width * 3 / 4, area.y, 1, area.height),
        buf,
    );

    j.render(
        Rect::new(
            area.x + area.width * 3 / 4 + 1,
            area.y,
            area.width / 4,
            area.height,
        ),
        buf,
    );
}

fn render_vertical_line(area: Rect, buf: &mut ratatui::prelude::Buffer) {
    let height = area.height as usize;
    let vertical_line: String = std::iter::repeat("│\n").take(height).collect();

    let line_paragraph = Paragraph::new(vertical_line).block(Block::default());

    line_paragraph.render(area, buf);
}
