use super::seek_bar;
use iced::Element;
use iced::Theme;

// State
#[derive(Default)]
pub struct MainApp {
    seek_bar: seek_bar::SeekBar,
}
#[derive(Debug, Clone)]
pub enum Message {
    SeekBar(seek_bar::Message),
}

impl MainApp {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SeekBar(message) => {
                // Pass the seek bar's update on to it
                self.seek_bar.update(message)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let current_seek_bar = seek_bar::SeekBar::view(&self.seek_bar);
        current_seek_bar.map(Message::SeekBar)
    }

    pub fn theme(&self) -> Theme {
        Theme::SolarizedDark
    }
}
