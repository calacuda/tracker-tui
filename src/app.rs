use crate::{
    event::{AppEvent, Event, EventHandler},
    Sequence, Sequences,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ScrollbarState,
    DefaultTerminal,
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// step number.
    pub step: u8,
    /// Event handler.
    pub events: EventHandler,
    /// all known/programmed sequences
    pub sequences: Sequences,
    /// the index of the active sequence
    pub displaying_sequences: Vec<usize>,
    /// is the app outputing midi data
    pub playing: bool,
    /// which sequences are playing.
    pub playing_sequences: Vec<usize>,
    /// the playback BPM
    pub bpm: usize,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            step: 0,
            events: EventHandler::new(),
            sequences: vec![Sequence::new("Sequence-0".into())],
            displaying_sequences: vec![0],
            playing: false,
            playing_sequences: Vec::new(),
            bpm: 120,
            vertical_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            // terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => {
                if let crossterm::event::Event::Key(key_event) = event {
                    self.handle_key_event(key_event)?;
                }
            }
            Event::App(app_event) => match app_event {
                // AppEvent::Increment => self.increment_counter(),
                // AppEvent::Decrement => self.decrement_counter(),
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            // KeyCode::Right => self.events.send(AppEvent::Increment),
            // KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    // pub fn increment_counter(&mut self) {
    //     self.counter = self.counter.saturating_add(1);
    // }
    //
    // pub fn decrement_counter(&mut self) {
    //     self.counter = self.counter.saturating_sub(1);
    // }
}
