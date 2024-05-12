use crate::tui;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::symbols;
use ratatui::widgets::{BorderType, List, ListState, Tabs};
use ratatui::{
    layout::Rect,
    style::Stylize,
    widgets::{Block, Borders},
    Frame,
};
use std::io;

use super::Component;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    selected_tab: Tab,
}

#[derive(Debug, Default)]
pub enum Tab {
    #[default]
    Themes,
}

impl Component for App {
    fn init(&mut self, area: Rect) -> Result<(), String> {
        Ok(())
    }

    fn handle_events(
        &mut self,
        event: Option<Event>,
    ) -> Result<Option<crate::action::Action>, String> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
            _ => None,
        };

        Ok(r)
    }

    fn handle_key_events(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<crate::action::Action>, String> {
        Ok(None)
    }

    fn handle_mouse_events(
        &mut self,
        mouse: event::MouseEvent,
    ) -> Result<Option<crate::action::Action>, String> {
        Ok(None)
    }

    fn update(
        &mut self,
        action: crate::action::Action,
    ) -> Result<Option<crate::action::Action>, String> {
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), String> {
        todo!()
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
    }
}
