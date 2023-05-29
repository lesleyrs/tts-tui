use arboard::Clipboard;
use std::error;
use tts::Tts;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    pub running: bool,
    pub clipboard: Clipboard,
    pub tts: Tts,
    pub pause: bool,
    pub history: Vec<String>,
    pub last_copy: String,
    pub text: String,
    pub tab_length: u16,
    pub selected: usize,
    pub line: u16,
    pub jump_length: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            clipboard: Clipboard::new().unwrap(),
            tts: Tts::default().unwrap(),
            pause: false,
            history: Vec::with_capacity(10),
            last_copy: String::from(""),
            text: String::from(""),
            tab_length: 3,
            selected: 0,
            line: 0,
            jump_length: 10,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        match self.clipboard.get_text() {
            Ok(contents) => {
                if self.pause {
                    if self.tts.is_speaking().unwrap() {
                        self.tts.stop().unwrap();
                    } else {
                        self.tts
                            .speak(
                                &self
                                    .text
                                    .chars()
                                    .filter(|&c| c != '\n' && c != '\r')
                                    .collect::<String>(),
                                true,
                            )
                            .unwrap();
                    }
                    self.pause = false;
                } else if self.last_copy != contents {
                    self.tts
                        .speak(
                            &contents
                                .chars()
                                .filter(|&c| c != '\n' && c != '\r')
                                .collect::<String>(),
                            true,
                        )
                        .unwrap();
                    self.last_copy = contents.clone();
                    self.text = contents.clone();
                    self.line = 0;
                    if self.history.len() > 9 {
                        self.history.pop();
                    }
                    self.history.insert(0, contents);
                    self.selected = 0;
                }
            }
            Err(_e) => (),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
