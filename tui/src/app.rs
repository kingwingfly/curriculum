#[path = "../../curriculum/common/common.rs"]
mod common;

use std::io;

use curriculum::{Course, DefaultTimeProvider, Query as _};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize as _},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarState, StatefulWidget, Widget, Wrap},
};

#[derive(Debug)]
pub struct App {
    time_provider: DefaultTimeProvider,
    exit: bool,
    courses_state: CoursesState,
}

#[derive(Debug)]
pub struct CoursesState {
    courses: Vec<Course>,
    position: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            time_provider: DefaultTimeProvider,
            exit: false,
            courses_state: CoursesState {
                courses: common::courses().filter_by_name(common::chosen()).collect(),
                position: 0,
            },
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            KeyCode::Up => {
                self.courses_state.position = self.courses_state.position.saturating_sub(1)
            }
            KeyCode::Down => {
                self.courses_state.position = (self.courses_state.position + 1)
                    .min(self.courses_state.courses.len().saturating_sub(1))
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Curriculum ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let chunks = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Fill(1),
            Constraint::Length(25),
        ])
        .split(block.inner(area));

        {
            let chunks =
                Layout::vertical([Constraint::Max(5), Constraint::Fill(1)]).split(chunks[0]);

            {
                let area = chunks[1];
                let block = Block::bordered().title("Courses").border_set(border::THICK);
                let chunks = Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)])
                    .split(block.inner(area));

                const LABEL_LINES: usize = 4;

                let lines = area.height as usize / LABEL_LINES;

                Scrollbar::default().render(
                    chunks[1],
                    buf,
                    &mut ScrollbarState::new(self.courses_state.courses.len())
                        .position(self.courses_state.position)
                        .viewport_content_length(lines),
                );

                let chunks = Layout::vertical(std::iter::repeat_n(
                    Constraint::Min(LABEL_LINES as u16),
                    lines,
                ))
                .split(chunks[0]);

                let skip = self.courses_state.position / lines * lines;
                self.courses_state
                    .courses
                    .iter()
                    .skip(skip)
                    .zip(chunks.iter())
                    .enumerate()
                    .for_each(|(i, (c, &area))| {
                        Paragraph::new(c.name().as_str())
                            .centered()
                            .wrap(Wrap { trim: false })
                            .style(if i + skip == self.courses_state.position {
                                Color::Green
                            } else {
                                Color::White
                            })
                            .render(area, buf);
                    });
                block.render(area, buf);
            }
        }

        block.render(area, buf);
    }
}
