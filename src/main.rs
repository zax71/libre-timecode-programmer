use anyhow::{Context, Result};

use components::main_app::MainApp;
use components::seek_bar::SeekBar;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
mod components;

fn main() -> iced::Result {
    // Get CLI arguments
    //let args = Args::parse();
    //play_audio(args.file).expect("Failed to play audio");

    iced::application("Libre Timecode Programmer", MainApp::update, MainApp::view)
        .theme(MainApp::theme)
        .run()
}

fn play_audio(location: PathBuf) -> Result<()> {
    // Get an output stream handle to the default physical sound device.
    // Note that no sound will be played if _stream is dropped
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(location)?);
    // Decode that sound file into a source
    let source = Decoder::new(file).context("Failed to read audio file")?;
    let duration = source
        .total_duration()
        .context("Audio file should have a duration")?;

    // Play the sound directly on the device
    stream_handle.play_raw(source.convert_samples())?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(duration);

    Ok(())
}
