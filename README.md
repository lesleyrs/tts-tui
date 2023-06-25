# TTS TUI

This is the terminal UI version of [Clipboard Narrator](https://github.com/lesleyrs/clipboard-narrator) but uses newer WinRT API to allow using any voice on windows.

## Comparison to Microsoft Edge TTS
Edge TTS | TTS-TUI
|---|---|
requires using MS Edge | works anywhere you can copy
uses cloud voices | uses system voices
delay each time you start a voice | little to no delay

Windows built-in narrator has a different use case but does use the system voices.

PDF readers don't allow copying newlines between paragraphs so they can't be split directly, convert it to a txt file with the original layout first.

## How to get
Install | Build | Binaries
|---|---|---|
cargo install tts-tui | cargo r --release | [Github releases](https://github.com/lesleyrs/tts-tui/releases/latest)

The binary is called `tts` for brevity

### Windows
Add voice packs or change the default voice in your system settings, your terminal title will show the active voice.

### Linux
The following deps are required:
- sudo apt install llvm-dev libclang-dev clang
  - https://rust-lang.github.io/rust-bindgen/requirements.html#debian-based-linuxes
- sudo apt install speech-dispatcher
- sudo apt install libspeechd-dev

On Debian you have to change the default feature based on `speech-dispatcher -v`:
  - --no-default-features --features 10
  - --no-default-features --features 9
