# tts-tui

Clipboard Narrator in the terminal

# Comparison to Microsoft Edge TTS
Edge TTS | tts-tui
|---|---|
requires using MS Edge | works with any program
online (even for offline files) | offline
delay each time you start a voice | no delay

Windows built-in narrator has a different use case but does use the system voices.

This is the terminal UI version of https://github.com/lesleyrs/clipboard-narrator but using WinRT to allow using any voice on windows.

The binary is called `tts` for brevity, pre-built binaries will be available soon.

## Windows
Add voice packs or change the default voice in your system settings.
### Install
- cargo install tts-tui

### Build
- cargo run --release

## Linux
These voices are not as good, sadly.
- sudo apt install llvm-dev libclang-dev clang
  - https://rust-lang.github.io/rust-bindgen/requirements.html#debian-based-linuxes
- sudo apt install speech-dispatcher
- sudo apt install libspeechd-dev

then one of the following based on `speech-dispatcher -v`
  ### Install
  - cargo install tts-tui
  - cargo install tts-tui --no-default-features --features 10
  - cargo install tts-tui --no-default-features --features 9
  ### Build
  - cargo run --release
  - cargo run --release --no-default-features --features 10
  - cargo run --release --no-default-features --features 9
