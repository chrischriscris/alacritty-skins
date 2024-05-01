use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style, Styled};
use ratatui::symbols::{self, border};
use ratatui::widgets::{List, ListState, Tabs};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
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
        // This should be stored outside of the function in your application state.
        let mut state = ListState::default();
        state.select(Some(self.counter as usize));

        let items = (1..10).map(|i| format!("Theme #{}", i.to_string()));
        let list = List::new(items)
            .block(
                Block::default()
                    .title("Select a theme from the list")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::new().underlined())
            .repeat_highlight_symbol(true);

        let area = Rect::new(0, 0, 8, 1);
        let tabs = Tabs::new(vec!["Themes"])
            .style(
                Style::default()
                    .bg(ratatui::style::Color::White)
                    .black()
                    .bold(),
            )
            .highlight_style(Style::default().yellow())
            .select(2)
            .divider(symbols::DOT);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1), Constraint::Percentage(100)])
            .split(frame.size())[1];

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40)])
            .split(layout);

        frame.render_widget(tabs, area);
        frame.render_stateful_widget(list, layout[0], &mut state);
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
            KeyCode::Down => self.decrement_counter(),
            KeyCode::Up => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Counter App Tutorial ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Down>".blue().bold(),
            " Increment ".into(),
            "<Up>".blue().bold(),
            " Quit ".into(),
            "<Esc> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
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
