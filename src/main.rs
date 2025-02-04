use anyhow::{Context, Result};
use iced::widget::{slider, Column};
use iced::Theme;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> iced::Result {
    // Get CLI arguments
    //let args = Args::parse();
    //play_audio(args.file).expect("Failed to play audio");

    iced::application("Libre Timecode Programmer", SeekBar::update, SeekBar::view)
        .theme(SeekBar::theme)
        .run()
}
#[derive(Default)]
pub struct SeekBar {
    time: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SliderChanged(f32),
}

impl SeekBar {
    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.time = x,
        }
    }

    fn view(&self) -> Column<Message> {
        let slider = slider(0.0..=100.0, self.time, Message::SliderChanged);

        let container = Column::new().push(slider);

        container
    }

    fn theme(&self) -> Theme {
        Theme::SolarizedDark
    }
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
