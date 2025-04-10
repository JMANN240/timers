use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, StatefulWidget, Widget},
    Frame,
};
use std::{io::Result, time::Duration};
use tui::Tui;
use tui_widgets::big_text::BigText;

mod tui;

pub struct Theme {
    fg: Color,
    bg: Color,
}

pub struct TimersState {
    theme: Theme,
}

#[derive(Default)]
pub struct Timers {
    exit: bool,
    running: bool,
    timer: Duration,
}

impl Timers {
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let state = &mut TimersState {
            theme: Theme {
                fg: Color::Yellow,
                bg: Color::Red,
            },
        };

        frame.render_stateful_widget(self, frame.area(), state);
    }

    fn handle_events(&mut self) -> Result<()> {
        let frame_rate = Duration::from_secs_f64(1.0 / 60.0);
        if event::poll(frame_rate)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Esc, KeyModifiers::NONE) => {
                            self.exit = true;
                        }
                        (KeyCode::Char(' '), KeyModifiers::NONE) => {
                            self.running = !self.running;
                        }
                        (KeyCode::Char('r'), KeyModifiers::NONE) => {
                            self.timer = Duration::new(0, 0);
                        }
                        _ => (),
                    }
                }
                _ => {}
            }
        }
        if self.running {
            self.timer += frame_rate;
        }
        Ok(())
    }
}

impl StatefulWidget for &mut Timers {
    type State = TimersState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let instructions = Line::from(vec![
            "Toggle ".into(),
            "<Space>".bold(),
            " Reset ".into(),
            "<R>".bold(),
            " Exit ".into(),
            "<Escape>".bold(),
        ]);

        let block = Block::new()
            .bg(state.theme.bg)
            .fg(state.theme.fg)
            .title_bottom(instructions.centered());

        let hours = self.timer.as_secs() / 60 / 60;
        let hours_string = format!("{hours:02}");
        let minutes = self.timer.as_secs() / 60 % 60;
        let minutes_string = format!("{minutes:02}");
        let seconds = self.timer.as_secs() % 60;
        let seconds_string = format!("{seconds:02}");
        let milliseconds = self.timer.subsec_millis();
        let milliseconds_string = format!("{milliseconds:03}");

        let timer_text = BigText::builder()
            .pixel_size(tui_widgets::big_text::PixelSize::Quadrant)
            .lines(vec![Line::from(vec![
                hours_string.into(),
                ":".into(),
                minutes_string.into(),
                ":".into(),
                seconds_string.into(),
                ".".into(),
                milliseconds_string.into(),
            ])])
            .centered()
            .build();

        timer_text.render(block.inner(center_vertical(area, 6)), buf);
        block.render(area, buf);
    }
}

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let mut timers = Timers::default();
    let timers_result = timers.run(&mut terminal);
    tui::restore()?;
    timers_result
}

fn center_vertical(area: Rect, height: u16) -> Rect {
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
