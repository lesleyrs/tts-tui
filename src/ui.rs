use crate::app::App;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let clipboard = app.clipboard.get_text().unwrap_or_default();
    frame.render_widget(
        Paragraph::new(&*clipboard)
            .wrap(Wrap { trim: false }) // true or false
            .block(
                Block::default()
                    .title(format!(
                        "{} chars {} words {} lines copied",
                        clipboard.chars().count(),
                        clipboard.split_whitespace().count(),
                        clipboard.lines().count()
                    ))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::White)),
            )
            .style(Style::default().fg(Color::LightYellow).bg(Color::Black))
            .alignment(Alignment::Center),
        frame.size(),
    )
}
