use arboard::Clipboard;
use std::error;
use tts::{Features, Tts};

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub static mut PARAGRAPH: usize = 0;
pub static mut VECTOR: Vec<&str> = Vec::new();
pub static mut COPY: String = String::new();
pub static mut LINE: u16 = 0;

pub struct App {
    pub running: bool,
    pub clipboard: Clipboard,
    pub tts: Tts,
    pub pause: bool,
    pub history: Vec<String>,
    pub last_copy: String,
    pub tab_length: u16,
    pub selected: usize,
    pub jump_length: u16,
    pub last_paragraph: usize,
    pub pdf_mode: bool,
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
            tab_length: 3,
            selected: 0,
            jump_length: 15,
            last_paragraph: 0,
            pdf_mode: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        let Features {
            utterance_callbacks,
            ..
        } = self.tts.supported_features();
        if utterance_callbacks {
            self.tts
                .on_utterance_end(Some(Box::new(|_utterance| unsafe {
                    if !VECTOR.is_empty() && PARAGRAPH < VECTOR.len() - 1 {
                        PARAGRAPH += 1;
                    }
                })))
                .unwrap();
        }
        match self.clipboard.get_text() {
            Ok(contents) => unsafe {
                // pause doesn't stop actions from starting tts intentionally
                if self.pause {
                    if self.tts.is_speaking().unwrap() {
                        self.tts.stop().unwrap();
                    } else {
                        match self.pdf_mode {
                            false => self.tts.speak(VECTOR[PARAGRAPH], true).unwrap(),
                            true => self
                                .tts
                                .speak(VECTOR[PARAGRAPH].replace('\n', " "), true)
                                .unwrap(),
                        };
                    }
                    self.pause = false;
                } else if self.last_copy != contents {
                    self.last_copy = contents.clone();
                    (self.last_paragraph, PARAGRAPH, LINE, self.selected) = (0, 0, 0, 0);
                    COPY = contents.chars().filter(|&c| c != '\r').collect();
                    VECTOR = COPY.split("\n\n").filter(|s| !s.is_empty()).collect();
                    if !VECTOR.is_empty() {
                        match self.pdf_mode {
                            false => self.tts.speak(VECTOR[PARAGRAPH], true).unwrap(),
                            true => self
                                .tts
                                .speak(VECTOR[PARAGRAPH].replace('\n', " "), true)
                                .unwrap(),
                        };
                    }
                    if self.history.len() > 9 {
                        self.history.pop();
                    }
                    self.history.insert(0, contents);
                } else if self.last_paragraph != PARAGRAPH {
                    self.last_paragraph = PARAGRAPH;
                    match self.pdf_mode {
                        false => self.tts.speak(VECTOR[PARAGRAPH], true).unwrap(),
                        true => self
                            .tts
                            .speak(VECTOR[PARAGRAPH].replace('\n', " "), true)
                            .unwrap(),
                    };
                }
            },
            Err(_e) => (),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
