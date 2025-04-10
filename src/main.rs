use std::{cell::RefCell, io::Result, ptr, rc::{Rc, Weak}, time::Duration};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{Block, Paragraph, Widget}, Frame};
use tui::Tui;
use tui_widgets::big_text::{self, BigText};

mod tui;

pub struct Timers {
	exit: bool,
	running: bool,
	timer: Duration
}

impl Timers {
	pub fn new() -> Timers {
		return Timers {
			exit: false,
			running: false,
			timer: Duration::new(0, 0)
		};
	}

	pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
		while !self.exit {
			terminal.draw(|frame| self.render_frame(frame))?;
			self.handle_events()?;
		}
		return Ok(());
	}

	fn render_frame(&mut self, frame: &mut Frame) {
		frame.render_widget(self, frame.area());
	}

	fn handle_events(&mut self) -> Result<()> {
		let frame_rate = Duration::from_secs_f64(1.0 / 60.0);
		if event::poll(frame_rate)? {
			match event::read()? {
				Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
					match (key_event.code, key_event.modifiers) {
						(KeyCode::Esc, KeyModifiers::NONE) => {
							self.exit = true;
						},
						(KeyCode::Char(' '), KeyModifiers::NONE) => {
							self.running = !self.running;
						},
						(KeyCode::Char('r'), KeyModifiers::NONE) => {
							self.timer = Duration::new(0, 0);
						},
						_ => ()
					}
				}
				_ => {}
			}
		}
		if self.running {
			self.timer += frame_rate;
		}
		return Ok(());
	}
}

impl Widget for &mut Timers {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let instructions = Line::from(vec![
			"Toggle ".into(),
			"<Space>".light_green().bold(),
			" Reset ".into(),
			"<R>".light_green().bold(),
			" Exit ".into(),
			"<Escape>".light_green().bold()
		]);

		let block = Block::bordered()
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
				milliseconds_string.into()
			])])
			.centered()
			.build();

		timer_text.render(block.inner(area), buf);
		block.render(area, buf);
	}
}

fn main() -> Result<()> {
	let mut terminal = tui::init()?;
	let mut timers = Timers::new();
	let timers_result = timers.run(&mut terminal);
	tui::restore()?;
	return timers_result;
}
