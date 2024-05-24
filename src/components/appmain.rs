use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::{Color, Style};
use ratatui::symbols;
use ratatui::widgets::Tabs;
use ratatui::{layout::Rect, style::Stylize, Frame};
use std::io;

use crate::tui;

#[derive(Debug)]
pub struct App {
    tabs: [AppTab; 1],
    selected_tab_index: usize,
    exit: bool,
}

#[derive(Debug)]
pub enum AppTab {
    Themes,
}

impl App {
    pub fn init() -> Self {
        Self {
            tabs: [AppTab::Themes],
            selected_tab_index: 0,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let area = Rect::new(0, 0, 8, 1);
        let tabs = Tabs::new(vec!["Themes"])
            .style(Style::new().bg(Color::Green).black())
            .highlight_style(Style::new().yellow())
            .select(2)
            .divider(symbols::DOT);
        frame.render_widget(tabs, area);

        // let canvas = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(vec![Constraint::Length(1), Constraint::Percentage(100)])
        //     .split(frame.size())[1];

        // let layout = Layout::default()
        //     .direction(Direction::Horizontal)
        //     .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
        //     .split(canvas);

        // let left = layout[0];
        // let right = layout[1];

        // let theme_selection = Layout::default()
        //     .direction(Direction::Vertical)
        //     .constraints(vec![Constraint::Length(3), Constraint::Percentage(100)])
        //     .split(left);

        // let theme_selection_filter = theme_selection[0];
        // let theme_selection_list = theme_selection[1];

        // frame.render_widget(
        //     Block::new()
        //         .title("")
        //         .title("🔍 (f) Filter")
        //         .borders(Borders::ALL)
        //         .border_type(BorderType::Rounded),
        //     theme_selection_filter,
        // );

        // // This should be stored outside of the function in your application state.
        // let mut state = ListState::default();
        // state.select(Some(self.counter as usize));

        // let items = (1..10).map(|i| format!("Theme #{}", i.to_string()));
        // let list = List::new(items)
        //     .block(
        //         Block::default()
        //             .title("")
        //             .title("🎨 Select theme")
        //             .title_style(Style::new().white())
        //             .borders(Borders::ALL)
        //             .border_style(Style::new().green())
        //             .border_type(BorderType::Rounded),
        //     )
        //     .highlight_style(Style::new().bg(Color::Cyan).black())
        //     .repeat_highlight_symbol(true);
        // frame.render_stateful_widget(list, theme_selection_list, &mut state);
        // frame.render_widget(
        //     Block::new()
        //         .title("")
        //         .title("👁️  Preview")
        //         .borders(Borders::ALL)
        //         .border_type(BorderType::Rounded),
        //     right,
        // );
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),

            KeyCode::Up => self.decrement_counter(),
            KeyCode::Down => self.increment_counter(),

            // Vim bindings
            KeyCode::Char('k') => self.decrement_counter(),
            KeyCode::Char('j') => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
