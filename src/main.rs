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
use std::{fs, io};
use toml_edit::DocumentMut;

mod alacritty;
mod tui;

#[derive(Debug, Default)]
pub struct App {
    counter: u32,
    exit: bool,
    tabs: AppTab,
    selected_tab: AppTab,
}

#[derive(Debug, Default)]
pub enum AppTab {
    #[default]
    Themes,
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
        let area = Rect::new(0, 0, 8, 1);
        let tabs = Tabs::new(vec!["Themes"])
            .style(Style::default().bg(Color::Green).black())
            .highlight_style(Style::default().yellow())
            .select(2)
            .divider(symbols::DOT);
        frame.render_widget(tabs, area);

        let canvas = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Percentage(100)])
            .split(frame.size())[1];

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(canvas);

        let left = layout[0];
        let right = layout[1];

        let theme_selection = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Percentage(100)])
            .split(left);

        let theme_selection_filter = theme_selection[0];
        let theme_selection_list = theme_selection[1];

        frame.render_widget(
            Block::default()
                .title("")
                .title("🔍 Search")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            theme_selection_filter,
        );

        // This should be stored outside of the function in your application state.
        let mut state = ListState::default();
        state.select(Some(self.counter as usize));

        let items = (1..10).map(|i| format!("Theme #{}", i.to_string()));
        let list = List::new(items)
            .block(
                Block::default()
                    .title("")
                    .title("🎨 Select theme")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::new().bg(Color::Cyan).bold())
            .repeat_highlight_symbol(true);
        frame.render_stateful_widget(list, theme_selection_list, &mut state);
        frame.render_widget(
            Block::default()
                .title("")
                .title("👁️  Preview")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            right,
        );
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

    fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1) % 10;
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}

fn try_main() -> Result<(), String> {
    let platform = alacritty::detect_platform()?;
    let config_file = alacritty::get_config_file_path(platform)?;
    let config_file = fs::read_to_string(config_file).expect("Unable to read file");

    let mut parsed = match config_file.parse::<DocumentMut>() {
        Ok(parsed) => parsed,
        Err(error) => return Err(format!("Failed to parse configuration file: {}", error)),
    };

    //  2.2 If it's not a toml return, only operate on toml

    // 3. Read the file, is it possible to keep it open?
    //    The idea is to have it continuosly open and write and save
    //    without closing it in a loop to make it a live preview

    // 4. Display a nice looking UI that lists all the available themes,
    //    lets you scroll through them, preview the colors and fuzzyfind some
    // println!("Select a theme: {:?}", platform);

    // 5. Add the theme to the config file
    let themes = alacritty::get_themes()?;
    themes.iter().for_each(|theme| {
        println!("{:?}", theme);
    });

    // 5. Return when escaping
    let imports = parsed["import"].as_array_mut().expect("Not an array ahaha");

    imports.push(format!(
        "~/Projects/alacritty-skins/themes/{}.toml",
        "gruvbox"
    ));

    // 6. Save the file
    // fs::write("alacritty.toml", parsed.to_string()).expect("Unable to write file");

    Ok(())
}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    let mut terminal = tui::init().unwrap();
    let app_result = App::default().run(&mut terminal);
    tui::restore().unwrap();

    app_result.unwrap();
}
