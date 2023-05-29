use crate::app::{App, COPY, PARAGRAPH, VECTOR};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::DOT,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let tabs = [
        "Tab 1", "Tab 2", "Tab 3", "Tab 4", "Tab 5", "Tab 6", "Tab 7", "Tab 8", "Tab 9", "Tab 10",
    ]
    .iter()
    .cloned()
    .map(Line::from)
    .collect();
    if frame.size().height < 3 {
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
            Rect::new(0, 0, frame.size().width, frame.size().height),
        );
    } else {
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
        unsafe {
            if !VECTOR.is_empty() {
                let (head, tail) = VECTOR.split_at(PARAGRAPH);
                let a = head.join("\n\n");
                let b = tail.get(1..tail.len()).unwrap().join("\n\n");
                let mut c = tail.first().unwrap().to_string();
                if PARAGRAPH != 0 {
                    c.insert(0, '\n');
                }
                c.push_str("\n\n");
                let h = a.split_inclusive(|c| c == '\n');
                let t = b.split_inclusive(|c| c == '\n');
                let m = c.split_inclusive(|c| c == '\n');
                let mut text = vec![];
                for l in h {
                    text.push(Line::from(l));
                }
                for l in m {
                    text.push(Line::from(Span::styled(
                        l,
                        Style::default().bg(Color::Blue),
                    )));
                }
                for l in t {
                    text.push(Line::from(l));
                }
                frame.render_widget(
                    Paragraph::new(text)
                        .wrap(Wrap { trim: true })
                        .block(
                            Block::default()
                                .title(format!(
                                    "{} chars {} words {} lines copied",
                                    COPY.chars().count(),
                                    COPY.split_whitespace().count(),
                                    COPY.lines().count()
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
        }
    }
}
