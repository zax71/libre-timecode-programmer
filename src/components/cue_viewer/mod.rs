mod cue;
mod cue_line;
mod text_cue;

use cue::Cue;
use iced::widget::{container, Container};
use iced::{Length, Padding};
use uuid::Uuid;
use vtc::{rates, Timecode};

#[derive(Default)]
pub struct CueViewer {
    cues: Vec<Cue>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddCue(Cue),
    RemoveCue(Uuid),
}

impl CueViewer {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddCue(cue) => self.cues.push(cue),
            // Removes the cue if the UUID is found in the vec, otherwise does nothing
            Message::RemoveCue(uuid) => {
                let found_cue_position = self
                    .cues
                    .iter()
                    .position(|current_cue| current_cue.id == uuid);
                match found_cue_position {
                    Some(cue_id) => {
                        self.cues.swap_remove(cue_id);
                    }
                    None => (),
                }
            }
        }
    }

    pub fn view(&self) -> Container<'_, Message> {
        // TODO: Render SeekBar::cues

        let cue = Cue {
            description: "This cue's description".to_string(),
            cue_id: "0.1.1".to_string(),
            start: Timecode::with_frames("01:02:00:05", rates::F30).unwrap(),
            end: Timecode::with_frames("01:02:03:05", rates::F30).unwrap(),
            id: Uuid::new_v4(),
        };

        let cue_container = container(cue.view())
            .padding(Padding::new(30.0))
            .center(Length::Fill)
            .style(container::rounded_box);

        cue_container
    }
}
