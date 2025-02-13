use iced::widget::{container, text, Container};
use uuid::Uuid;
use vtc::Timecode;

#[derive(Debug, Clone)]
pub struct Cue {
    pub start: Timecode,
    pub end: Timecode,
    pub description: String,
    pub cue_id: String,
    pub id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Message {}

impl Cue {
    pub fn update(&mut self, _message: Message) {}

    pub fn view(&self) -> Container<Message> {
        let cue_text = format!("{} | {}", self.cue_id, self.description);
        container(text(cue_text)).padding(2)
    }
}
