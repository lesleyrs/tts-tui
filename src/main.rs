use crossterm::terminal::LeaveAlternateScreen;
use std::{io, process};
use tts_tui::app::{App, AppResult};
use tts_tui::event::{Event, EventHandler};
use tts_tui::handler::handle_key_events;
use tts_tui::tui::Tui;
use tui::backend::CrosstermBackend;
use tui::Terminal;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO: &str = env!("CARGO_PKG_REPOSITORY");

fn main() -> AppResult<()> {
    let mut args = std::env::args();
    match args.nth(1) {
        Some(arg) if arg == "--version" || arg.starts_with("-V") => {
            println!("{NAME} {VERSION}\n");
            process::exit(0);
        }
        Some(arg) if arg == "--help" || arg.starts_with("-h") => {
            println!("{NAME} {VERSION}");
            println!(env!("CARGO_PKG_DESCRIPTION"));
            println!("{}/releases/tag/{VERSION}\n", REPO);
            println!("The only options are --version and --help\n");
            println!("USAGE:");
            println!("  <space>\n  \tToggle speech");
            println!("  <s>\n  \tStop speech");
            println!("  <number>\n  \tCopy history");
            println!("  <up/k>\n  \tScroll up");
            println!("  <down/j>\n  \tScroll down");
            println!();
            process::exit(0);
        }
        Some(arg) if arg.starts_with('-') => {
            if arg.starts_with("--") {
                eprintln!(
                    "error: unexpected argument '{}' found",
                    arg.split_whitespace().next().unwrap(),
                );
            } else if arg.starts_with('-') {
                eprintln!(
                    "error: unexpected argument '{}' found",
                    arg.get(..2).get_or_insert("-")
                );
            }
            eprintln!("\n  try --help\n");
            process::exit(0);
        }
        _ => {}
    }
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic);
    }));

    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init(&app)?;
    app.tts.set_rate(1.5).unwrap();
    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
