use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::crossterm::event;
use ratatui::text::Line;
use ratatui::widgets::Widget;
use ratatui::{prelude, DefaultTerminal, Frame};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App { exit: false };

    let app_result = app.run(&mut terminal);
    // Get CLI arguments
    //let args = Args::parse();
    //play_audio(args.file).expect("Failed to play audio");
    ratatui::restore();
    app_result
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

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            if event::poll(Duration::from_millis(250)).context("Event poll failed")? {
                match event::read()? {
                    crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                    _ => {}
                }
            }
            terminal
                .draw(|frame| self.draw(frame))
                .context("Failed to draw a frame")?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: prelude::Rect, buf: &mut prelude::Buffer)
    where
        Self: Sized,
    {
        // Line at top
        Line::from("Hello World").render(area, buf);
    }
}
