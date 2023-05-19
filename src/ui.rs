use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::DOT,
    text::Spans,
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let tabs = [
        "Tab 1", "Tab 2", "Tab 3", "Tab 4", "Tab 5", "Tab 6", "Tab 7", "Tab 8", "Tab 9", "Tab 10",
    ]
    .iter()
    .cloned()
    .map(Spans::from)
    .collect();
    frame.render_widget(
        Tabs::new(tabs)
            .select(app.selected)
            .block(
                Block::default()
                    .title(format!("History size: {}", app.history.len()))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT),
        Rect::new(0, 0, frame.size().width, app.tab_length),
    );
    frame.render_widget(
        Paragraph::new(&*app.text)
            .wrap(Wrap { trim: true })
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
            .alignment(Alignment::Center)
            .scroll((app.line, 0)),
        Rect::new(
            frame.size().x,
            frame.size().y + app.tab_length,
            frame.size().width,
            frame.size().height - app.tab_length,
        ),
    )
}
