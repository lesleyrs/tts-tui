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
            KeyCode::Char(char) if char.is_ascii_digit() => match char {
                '0' => {
                    if app.history.len() == 10 {
                        app.tts.speak(&app.history[9], true).unwrap();
                        app.text = app.history[9].clone();
                    }
                }
                _ => {
                    if app.history.len() >= char as usize - 0x30 {
                        app.tts
                            .speak(&app.history[char as usize - 0x31], true)
                            .unwrap();
                        app.text = app.history[char as usize - 0x31].clone();
                    }
                }
            },
            _ => {}
        }
    }
    Ok(())
}
