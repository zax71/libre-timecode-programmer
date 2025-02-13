use super::cue_viewer;
use super::seek_bar;
use iced::widget::column;
use iced::widget::container;
use iced::widget::Container;
use iced::Element;
use iced::Theme;

// State
#[derive(Default)]
pub struct MainApp {
    seek_bar: seek_bar::SeekBar,
    cue_viewer: cue_viewer::CueViewer,
}
#[derive(Debug, Clone)]
pub enum Message {
    SeekBar(seek_bar::Message),
    CueViewer(cue_viewer::Message),
}

impl MainApp {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SeekBar(message) => {
                // Pass the seek bar's update on to it
                self.seek_bar.update(message)
            }
            Message::CueViewer(message) => self.cue_viewer.update(message),
        }
    }

    pub fn view(&self) -> Container<Message> {
        let seek_bar = Element::from(seek_bar::SeekBar::view(&self.seek_bar)).map(Message::SeekBar);
        let cue_viewer =
            Element::from(cue_viewer::CueViewer::view(&self.cue_viewer)).map(Message::CueViewer);

        let pre_padding = column![seek_bar, cue_viewer];

        container(pre_padding).padding(10.0)
    }

    pub fn theme(&self) -> Theme {
        Theme::SolarizedDark
    }
}
