use crate::app::App;

pub mod app;
pub mod event;
pub mod midi_out;
pub mod ui;

pub type MidiNote = u8;
pub type MidiVelocity = u8;
// pub type Sequences = Vec<Option<(MidiNote, MidiVelocity)>>;
pub type Sequences = Vec<Sequence>;

#[derive(Debug, Clone)]
pub struct MidiOut {
    pub dev: String,
    pub channel: u8,
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub notes: Vec<Option<(MidiNote, MidiVelocity)>>,
    pub name: String,
    pub midi_out: Option<MidiOut>,
}

impl Sequence {
    pub fn new(name: String) -> Self {
        let notes = vec![
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];

        Self {
            notes,
            name,
            midi_out: None,
        }
    }

    pub fn len(&self) -> usize {
        self.notes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notes
            .clone()
            .iter()
            .filter(|elm| elm.is_some())
            .collect::<Vec<_>>()
            .is_empty()
            || self.notes.is_empty()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}
