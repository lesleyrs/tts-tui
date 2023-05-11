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
    pub text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            clipboard: Clipboard::new().unwrap(),
            tts: Tts::default().unwrap(),
            pause: false,
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
                        self.tts.speak(&contents, true).unwrap(); // add actual pause
                        self.text = contents;
                    }
                    self.pause = false;
                } else if self.text != contents {
                    self.tts.speak(&contents, true).unwrap();
                    self.text = contents;
                }
            }
            Err(_e) => (), // log the errors
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
