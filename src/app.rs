use arboard::Clipboard;
use std::error;
use tts::Tts;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// #[derive(Debug)]
pub struct App {
    pub running: bool,
    pub clipboard: Clipboard,
    pub tts: Tts,
    pub pause: bool,
    pub history: Vec<String>,
    pub last_copy: String,
    pub text: String,
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
                        self.tts.speak(&self.text, true).unwrap();
                    }
                    self.pause = false;
                } else if self.last_copy != contents {
                    self.tts.speak(&contents, true).unwrap();
                    self.last_copy = contents.clone();
                    self.text = contents.clone();
                    if self.history.len() > 9 {
                        self.history.pop();
                    }
                    self.history.insert(0, contents);
                }
            }
            Err(_e) => (),
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
    // let Features {
    //     utterance_callbacks,
    //     ..
    // } = tts.supported_features();
    // if utterance_callbacks {
    // tts.on_utterance_begin(Some(Box::new(|utterance| {
    // println!("Started speaking {:?}", utterance)
    // })))?;
    // tts.on_utterance_end(Some(Box::new(|utterance| {
    // println!("Finished speaking {:?}", utterance)
    // })))?;
    // tts.on_utterance_stop(Some(Box::new(|utterance| {
    // println!("Stopped speaking {:?}", utterance)
    // })))?;
    // }
}
