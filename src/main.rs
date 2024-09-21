use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

mod command;
use command::systemd_health::systemd_health;

fn main() -> Result<(), io::Error> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::start(terminal);
    ratatui::restore();
    app_result
}

enum App {
    Running(RunningApp),
    Closing,
}

#[derive(Default)]
struct RunningApp {
    run_count: u32,
}

impl App {
    fn start(mut terminal: DefaultTerminal) -> io::Result<()> {
        Self::Running(RunningApp::default()).run(&mut terminal)
    }

    // Run the application's main loop until the state change to closing
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while let Self::Running(app) = self.take() {
            terminal.draw(|frame| app.draw(frame))?;
            *self = app.handle_event()?;
        }
        Ok(())
    }

    fn take(&mut self) -> Self {
        let mut result = Self::Closing;
        std::mem::swap(self, &mut result);
        result
    }
}

impl RunningApp {
    fn handle_event(mut self) -> io::Result<App> {
        self.run_count = self.run_count.checked_add(1).unwrap();
        let res = match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => App::Running(self),
        };
        Ok(res)
    }
    fn handle_key_event(self, key: KeyEvent) -> App {
        if key.code == KeyCode::Char('q') {
            App::Closing
        } else {
            App::Running(self)
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &RunningApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Remote Execution Automation Tool ".blue().bold());
        let instruction = Title::from(Line::from(vec![
            "[ Quit: ".into(),
            "<Q>".red().bold(),
            " ]".into(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Center).position(Position::Top))
            .title(
                instruction
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED);

        let text = format!(
            "redrawn counter: {}\nmarisa systemd status: {}\nremote-port-forward systemd status: {}",
            self.run_count,
            systemd_health("marisa").to_color(),
            systemd_health("remote-port-forward").to_color()
        );

        Paragraph::new(Text::from(text))
            .left_aligned()
            .block(block)
            .render(area, buf);
    }
}
