use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::scrollbar::VERTICAL,
    widgets::{Block, BorderType, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation},
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
            // Constraint::Ratio(1, 3),
            Constraint::Min(16),
            // TODO: one for every sequence being viewed.
            Constraint::Min(12),
            // Constraint::Min(16),
            // Constraint::Ratio(1, 3),
            // Constraint::Ratio(1, 3),
        ])
        .split(area);

        // sequence list (with default midi output and channel displayed)
        self.sequences_display(frame, layout[0]);
        // sequence data (the notes) with the current step highlighted.
        // TODO: one for every sequence being viewed.
        self.sequence_display(frame, layout[1])
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
                    .title_top(selected.name),
            ),
            area,
        );
    }

    pub fn sequences_display(&mut self, frame: &mut Frame, area: Rect) {
        // let create_block = |title: &'static str| Block::bordered().gray().title(title.bold());
        let selected = self.displaying_sequence;
        self.vertical_scroll_state = self
            .vertical_scroll_state
            .content_length(self.sequences.len());
        let seqs: Vec<String> = self
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
                    "* ".to_string()
                } else {
                    "".to_string()
                };

                format!(">  {prefix}{}\n   {out_name} => {out_chan}", seq.name)
                // .block(create_block("Vertical scrollbar with arrows"))
            })
            .collect();
        let text = Paragraph::new(seqs.join("\n\n"));

        frame.render_widget(
            // List::new(seqs)
            text.block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Sequences"),
            )
            .scroll((self.vertical_scroll as u16, 0)),
            area,
        );

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .symbols(VERTICAL)
                .track_symbol(None)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut self.vertical_scroll_state,
        );
    }

    pub fn build_step_view(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::horizontal([
            Constraint::Min(6),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
            Constraint::Min(3),
        ])
        .split(area);

        frame.render_widget(
            Paragraph::new(format!("{}", self.bpm))
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title("BPM"), // .border_style(Color::Reset),
                )
                // .fg(color)
                .centered(),
            layout[0],
        );

        for i in 1..=16 {
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
