// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2020 7sDream <i@7sdre.am> and contributors
//
// This file is part of FontFor.
//
// FontFor is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod canvas_render;
mod event;
mod state;

use {
    crate::{font::SortedFamilies, ft::Library as FtLibrary},
    canvas_render::CanvasRenderResult,
    crossterm::{
        event::{KeyCode as CtKeyCode, KeyModifiers as CtKM},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        Result as CTResult,
    },
    event::{TerminalEvent, TerminalEventStream},
    state::{RenderType, State},
    std::{
        io::{Stdout, Write},
        ops::DerefMut,
        time::Duration,
    },
    tui::{
        backend::CrosstermBackend,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::{Color, Modifier, Style},
        terminal::{Frame, Terminal},
        widgets::{canvas::Canvas, Block, Borders, List, Paragraph, Text},
    },
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum OnEventResult {
    ReDraw,
    Continue,
    Exit,
}

pub struct UI<'fc, 'ft> {
    idle_redraw: u8,
    state: State<'fc, 'ft>,
}

impl<'fc, 'ft> UI<'fc, 'ft> {
    pub fn new(c: char, families: SortedFamilies<'fc>, ft: &'ft mut FtLibrary) -> Option<Self> {
        if families.len() > 0 {
            Some(Self { state: State::new(c, families, ft), idle_redraw: 0 })
        } else {
            None
        }
    }

    fn draw_list<B>(&self, area: Rect, f: &mut Frame<'_, B>)
    where
        B: tui::backend::Backend,
    {
        let families = self.state.family_names();
        let index = self.state.index();
        let title = format!("Fonts {}/{}", index + 1, families.len());

        let list = List::new(families.iter().copied().map(Text::raw))
            .block(Block::default().title(&title).borders(Borders::ALL))
            .highlight_style(Style::default().fg(Color::LightBlue).modifier(Modifier::BOLD));

        f.render_stateful_widget(list, area, self.state.mut_list_state().deref_mut())
    }

    fn draw_canvas<B>(&self, area: Rect, f: &mut Frame<'_, B>)
    where
        B: tui::backend::Backend,
    {
        let result = self.state.render();

        if self.state.get_render_type() == &RenderType::Mono && result.is_ok() {
            // Mono render to canvas
            let (canvas_width, canvas_height) = self.state.get_char_pixel_cell();
            let canvas_width = f64::from(canvas_width);
            let canvas_height = f64::from(canvas_height);
            let canvas = Canvas::default()
                .block(Block::default().title("Preview").borders(Borders::ALL))
                .x_bounds([0.0, canvas_width - 1.0])
                .y_bounds([0.0, canvas_height - 1.0])
                .paint(|ctx| {
                    let chars = result.as_ref().as_ref().unwrap();
                    let shape = CanvasRenderResult::new(chars, canvas_width, canvas_height);
                    ctx.draw(&shape);
                });
            f.render_widget(canvas, area);
        } else {
            // Others render to paragraph
            let (height, result) = match result.as_ref() {
                Ok(result) => (result.height(), result.to_string()),
                Err(err) => (err.lines().count(), (*err).to_string()),
            };

            let padding = (area.height as usize).saturating_sub(2).saturating_sub(height);
            let padding_lines = "\n".repeat(padding / 2);

            let texts = [Text::raw(padding_lines), Text::raw(result)];

            let canvas = Paragraph::new(texts.iter())
                .block(Block::default().title("Preview").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Reset).modifier(Modifier::BOLD))
                .wrap(false);
            f.render_widget(canvas, area);
        }
    }

    fn generate_help_text<'a>(key: &'a str, help: &'a str) -> Vec<Text<'a>> {
        vec![
            Text::styled(key, Style::default().fg(Color::Cyan).modifier(Modifier::BOLD)),
            Text::raw(": "),
            Text::styled(help, Style::default().fg(Color::Blue).modifier(Modifier::BOLD)),
        ]
    }

    fn draw_status_bar_info<B>(&self, area: Rect, f: &mut Frame<'_, B>)
    where
        B: tui::backend::Backend,
    {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
            .split(area);

        let name = cols[0];
        let mode = cols[1];

        let texts = vec![
            Text::styled("Font Face", Style::default().fg(Color::Green)),
            Text::raw(": "),
            Text::styled(
                self.state.current_name(),
                Style::default().fg(Color::Blue).modifier(Modifier::BOLD),
            ),
        ];
        f.render_widget(
            Paragraph::new(texts.iter())
                .block(Block::default().title("Info").borders(Borders::TOP | Borders::LEFT))
                .alignment(Alignment::Left)
                .wrap(false),
            name,
        );

        let texts = vec![
            Text::styled("Render Mode", Style::default().fg(Color::Green)),
            Text::raw(": "),
            Text::styled(
                format!("{:?}", self.state.get_render_type()),
                Style::default().fg(Color::Blue).modifier(Modifier::BOLD),
            ),
        ];
        f.render_widget(
            Paragraph::new(texts.iter())
                .block(Block::default().borders(Borders::TOP | Borders::RIGHT))
                .alignment(Alignment::Right)
                .wrap(false),
            mode,
        );
    }

    fn draw_status_bar_help<B>(area: Rect, f: &mut Frame<'_, B>)
    where
        B: tui::backend::Backend,
    {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [Constraint::Ratio(2, 5), Constraint::Ratio(2, 5), Constraint::Ratio(1, 5)]
                    .as_ref(),
            )
            .split(area);

        let mut list_help = Self::generate_help_text("[Up]", "Prev Font ");
        list_help.append(&mut Self::generate_help_text("[Down]", "Next Font"));

        let mut mode_help = Self::generate_help_text("[Left]", "Prev Mode ");
        mode_help.append(&mut Self::generate_help_text("[Right]", "Next Mode"));

        let quit_help = Self::generate_help_text("[Q]", "Quit");

        f.render_widget(
            Paragraph::new(list_help.iter())
                .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT))
                .alignment(Alignment::Left)
                .wrap(false),
            cols[0],
        );

        f.render_widget(
            Paragraph::new(mode_help.iter())
                .block(Block::default().borders(Borders::BOTTOM))
                .alignment(Alignment::Center)
                .wrap(false),
            cols[1],
        );

        f.render_widget(
            Paragraph::new(quit_help.iter())
                .block(Block::default().borders(Borders::BOTTOM | Borders::RIGHT))
                .alignment(Alignment::Right)
                .wrap(false),
            cols[2],
        );
    }

    fn draw_status_bar<B>(&self, area: Rect, f: &mut Frame<'_, B>)
    where
        B: tui::backend::Backend,
    {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Length(2)].as_ref())
            .split(area);

        let info = rows[0];
        let help = rows[1];

        self.draw_status_bar_info(info, f);
        Self::draw_status_bar_help(help, f);
    }

    fn draw<B>(&self, f: &mut Frame<'_, B>)
    where
        B: tui::backend::Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(4)].as_ref())
            .split(f.size());

        let main = layout[0];
        let status_bar = layout[1];

        #[allow(clippy::cast_possible_truncation)] // never truncation because we `min` with 24
        let list_width = self.state.name_width_max().min(32) as u16;

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(list_width), Constraint::Min(24)].as_ref())
            .split(main);

        let list = main[0];
        let canvas = main[1];

        let mut width = u32::from(canvas.width.saturating_sub(2));
        let mut height = u32::from(canvas.height.saturating_sub(2));
        let rt = self.state.get_render_type();
        if rt == &RenderType::Moon {
            width /= 2;
        } else if rt == &RenderType::Mono {
            width = width.saturating_mul(2);
            height = height.saturating_mul(4);
        }

        self.state.update_char_pixel_cell(width, height);

        self.draw_list(list, f);
        self.draw_canvas(canvas, f);
        self.draw_status_bar(status_bar, f);
    }

    fn on_event(&mut self, event: CTResult<TerminalEvent>) -> CTResult<OnEventResult> {
        match event? {
            TerminalEvent::Tick => {
                self.idle_redraw += 1;
                Ok(if self.idle_redraw == 10 {
                    OnEventResult::ReDraw
                } else {
                    OnEventResult::Continue
                })
            }
            TerminalEvent::Key(key) => {
                if key.modifiers.contains(CtKM::ALT) || key.modifiers.contains(CtKM::CONTROL) {
                    Ok(OnEventResult::Continue)
                } else {
                    match key.code {
                        CtKeyCode::Char('q') => Ok(OnEventResult::Exit),
                        CtKeyCode::Up | CtKeyCode::Char('k') => {
                            self.state.move_up();
                            Ok(OnEventResult::ReDraw)
                        }
                        CtKeyCode::Down | CtKeyCode::Char('j') => {
                            self.state.move_down();
                            Ok(OnEventResult::ReDraw)
                        }
                        CtKeyCode::Left | CtKeyCode::Char('h') => {
                            self.state.prev_render_type();
                            Ok(OnEventResult::ReDraw)
                        }
                        CtKeyCode::Right | CtKeyCode::Char('l') => {
                            self.state.next_render_type();
                            Ok(OnEventResult::ReDraw)
                        }
                        _ => Ok(OnEventResult::Continue),
                    }
                }
            }
        }
    }

    fn setup() -> CTResult<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(terminal)
    }

    fn shutdown(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> CTResult<()> {
        terminal.show_cursor()?;
        let backend = terminal.backend_mut();
        execute!(backend, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn show(mut self) -> CTResult<()> {
        let mut terminal = Self::setup()?;

        let events = TerminalEventStream::new(Duration::from_millis(1000 / 60));

        terminal.draw(|mut f| self.draw(&mut f))?;

        for event in events.iter() {
            match self.on_event(event) {
                Err(kind) => {
                    Self::shutdown(terminal)?;
                    return Err(kind);
                }
                Ok(result) => match result {
                    OnEventResult::ReDraw => {
                        terminal.draw(|mut f| self.draw(&mut f))?;
                        self.idle_redraw = 0;
                    }
                    OnEventResult::Continue => (),
                    OnEventResult::Exit => {
                        return Self::shutdown(terminal);
                    }
                },
            }
        }

        Ok(())
    }
}
