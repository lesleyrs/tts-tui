use crate::app::{App, AppResult, LINE};
use crate::app::{PARAGRAPH, VECTOR};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind == KeyEventKind::Press {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            KeyCode::Char('s') => {
                if app.tts.is_speaking()? {
                    app.tts.stop()?;
                }
                app.pause = false;
            }
            KeyCode::Char(' ') => {
                app.pause = !app.pause;
            }
            KeyCode::Up | KeyCode::Char('k') => unsafe {
                LINE = LINE.saturating_sub(app.jump_length);
            },
            KeyCode::Down | KeyCode::Char('j') => unsafe {
                LINE = LINE.saturating_add(app.jump_length);
            },
            KeyCode::Left | KeyCode::Char('h') => unsafe {
                PARAGRAPH = PARAGRAPH.saturating_sub(1);
            },
            KeyCode::Right | KeyCode::Char('l') => unsafe {
                if !VECTOR.is_empty() && PARAGRAPH < VECTOR.len() - 1 {
                    PARAGRAPH += 1;
                }
            },
            KeyCode::Char(char) if char.is_ascii_digit() => match char {
                '0' => unsafe {
                    if app.history.len() == 10 {
                        app.selected = 9;
                        // app.tts.speak(&app.history[app.selected], true).unwrap();
                        // COPY = app.history[app.selected].clone();
                        PARAGRAPH = 0;
                        LINE = 0;
                    }
                },
                _ => unsafe {
                    if app.history.len() >= char as usize - 0x30 {
                        app.selected = char as usize - 0x31;
                        // app.tts.speak(&app.history[app.selected], true).unwrap();
                        // COPY = app.history[app.selected].clone();
                        PARAGRAPH = 0;
                        LINE = 0;
                    }
                },
            },
            _ => {}
        }
    }
    Ok(())
}
