use crate::app::App;
use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    frame.render_widget(
        Paragraph::new(&*app.text)
            .wrap(Wrap { trim: false }) // true or false
            .block(
                Block::default()
                    .title(format!(
                        "{} chars {} words {} lines copied",
                        app.text.chars().count(),
                        app.text.split_whitespace().count(),
                        app.text.lines().count()
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
