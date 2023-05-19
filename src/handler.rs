use crate::app::{App, AppResult};
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
                app.tts.stop()?;
                app.pause = false;
            }
            KeyCode::Char(' ') => {
                app.pause = !app.pause;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.line = app.line.saturating_sub(10);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.line = app.line.saturating_add(10);
            }
            KeyCode::Char(char) if char.is_ascii_digit() => match char {
                '0' => {
                    if app.history.len() == 10 {
                        app.selected = 9;
                        app.tts.speak(&app.history[app.selected], true).unwrap();
                        app.text = app.history[app.selected].clone();
                    }
                }
                _ => {
                    if app.history.len() >= char as usize - 0x30 {
                        app.selected = char as usize - 0x31;
                        app.tts.speak(&app.history[app.selected], true).unwrap();
                        app.text = app.history[app.selected].clone();
                    }
                }
            },
            _ => {}
        }
    }
    Ok(())
}
