use std::io;

use tui::{
  backend::CrosstermBackend,
  layout::Rect,
  style::{Color, Style},
  widgets::{Block, BorderType, Borders, Clear, Paragraph},
  Frame,
};

use crate::theme::Theme;

type Backend = CrosstermBackend<io::Stdout>;

pub fn render_remove_proc(area: Rect, frame: &mut Frame<Backend>) {
  let theme = Theme::default();

  let y = area.height / 2;
  let x = (area.width / 2).saturating_sub(20).max(1);

  let block = Block::default()
    .borders(Borders::ALL)
    .border_style(theme.pane_border(true))
    .border_type(BorderType::Rounded)
    .style(Style::default().bg(Color::Black));
  frame.render_widget(block, Rect::new(x - 1, y - 1, 42, 3).intersection(area));

  let txt = Paragraph::new("Remove process? (y/n)");
  let txt_area = Rect::new(x, y, 40, 1).intersection(area);
  frame.render_widget(Clear, txt_area);
  frame.render_widget(txt, txt_area);
}
