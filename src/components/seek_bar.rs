use iced::widget::{slider, Column};
use iced::Theme;

#[derive(Default)]
pub struct SeekBar {
    time: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SliderChanged(f32),
}

impl SeekBar {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.time = x,
        }
    }

    pub fn view(&self) -> Column<Message> {
        let slider = slider(0.0..=100.0, self.time, Message::SliderChanged);

        let container = Column::new().push(slider);

        container
    }
}
