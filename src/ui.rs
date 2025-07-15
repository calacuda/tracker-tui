use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn display_note(note: u8) -> String {
    let octave = note / 12;
    let note = note % 12;

    let names = [
        "C-", "C#", "D-", "D#", "E-", "F-", "F#", "G-", "G#", "A-", "A#", "B-",
    ];

    format!("{}{octave:.0X}", names[note as usize])
}

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let layout =
            Layout::vertical([Constraint::Length(3), Constraint::Min(18)]).split(frame.area());

        self.build_step_view(frame, layout[0]);
        self.main_section(frame, layout[1]);
    }

    pub fn main_section(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::horizontal([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(area);

        // sequence list (with default midi output and channel displayed)
        self.sequences_display(frame, layout[0]);
        // sequence data (the notes) with the current step highlighted.
        self.sequence_display(frame, layout[1])
        // TODO: settings window.
        //  - BPM,
    }

    pub fn sequence_display(&mut self, frame: &mut Frame, area: Rect) {
        let selected = self.sequences[self.displaying_sequence].clone();
        let seqs: Vec<ListItem> = selected
            .notes
            .iter()
            .enumerate()
            .map(|(i, note)| {
                let note = if let Some(note) = note {
                    display_note(note.0)
                } else {
                    "---".into()
                };

                ListItem::new(format!("{i:02.0X}: {note}"))
            })
            .collect();

        frame.render_widget(
            List::new(seqs).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Sequences"),
            ),
            area,
        );
    }

    pub fn sequences_display(&mut self, frame: &mut Frame, area: Rect) {
        let selected = self.displaying_sequence;
        let seqs: Vec<ListItem> = self
            .sequences
            .iter()
            .enumerate()
            .map(|(i, seq)| {
                let (out_name, out_chan) = if let Some(midi_out) = seq.midi_out.clone() {
                    (midi_out.dev, midi_out.channel)
                } else {
                    ("Default".into(), 0)
                };
                let prefix = if i == selected {
                    " * ".to_string()
                } else {
                    "".to_string()
                };

                ListItem::new(format!("{}{prefix}\n{out_name} => {out_chan}", seq.name))
            })
            .collect();

        frame.render_widget(
            List::new(seqs).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Sequences"),
            ),
            area,
        );
    }

    pub fn build_step_view(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::horizontal([
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
            Constraint::Ratio(1, 16),
        ])
        .split(area);

        for i in 0..16 {
            let color = if i == self.step && self.playing {
                Color::Green
            } else {
                Color::Reset
            };

            frame.render_widget(
                Paragraph::new(format!("{i:.0X}"))
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .border_style(color),
                    )
                    .fg(color)
                    .centered(),
                layout[i as usize],
            );
        }
    }
}
