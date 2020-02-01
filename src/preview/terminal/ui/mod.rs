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

use crate::preview::terminal::ui::state::RenderType;
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
    state::State,
    std::{
        io::{Stdout, Write},
        time::Duration,
    },
    tui::{
        backend::CrosstermBackend,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        terminal::{Frame, Terminal},
        widgets::{canvas::Canvas, Block, Borders, Paragraph, SelectableList, Text, Widget},
    },
};

enum OnEventResult {
    ReDraw,
    Continue,
    Exit,
}

pub struct UI<'fc, 'ft> {
    state: State<'fc, 'ft>,
}

#[allow(clippy::unused_self)] // TODO: use them
impl<'fc, 'ft> UI<'fc, 'ft> {
    pub fn new(c: char, families: SortedFamilies<'fc>, ft: &'ft mut FtLibrary) -> Option<Self> {
        if families.len() > 0 {
            Some(Self { state: State::new(c, families, ft) })
        } else {
            None
        }
    }

    fn draw_list<B>(&self, area: Rect, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        SelectableList::default()
            .block(Block::default().title("Fonts").borders(Borders::ALL))
            .items(self.state.family_names())
            .select(Some(self.state.index()))
            .highlight_symbol(">>")
            .render(f, area);
    }

    fn draw_canvas<B>(&self, area: Rect, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let result = self.state.render();

        if self.state.get_render_type() == &RenderType::Mono && result.is_ok() {
            let (canvas_width, canvas_height) = self.state.get_rect();
            let canvas_width = f64::from(canvas_width) * 2.0;
            let canvas_height = f64::from(canvas_height) * 4.0;
            Canvas::default()
                .block(Block::default().title("Preview").borders(Borders::ALL))
                .x_bounds([0.0, canvas_width - 1.0])
                .y_bounds([0.0, canvas_height - 1.0])
                .paint(|ctx| {
                    let chars = result.as_ref().as_ref().unwrap();
                    let shape = CanvasRenderResult::new(chars, canvas_width, canvas_height);
                    ctx.draw(&shape);
                })
                .render(f, area);
        } else {
            // Mono render to canvas
            let (height, result) = match result.as_ref() {
                Ok(result) => (result.height(), result.to_string()),
                Err(err) => (err.lines().count(), (*err).to_string()),
            };

            let padding = (area.height as usize).saturating_sub(2).saturating_sub(height);
            let padding_lines = "\n".repeat(padding / 2);

            let texts = [Text::raw(padding_lines), Text::raw(result)];

            Paragraph::new(texts.iter())
                .block(Block::default().title("Preview").borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(false)
                .render(f, area);
        }
    }

    fn draw_info<B>(&self, area: Rect, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let mut texts = Vec::new();
        texts.push(Text::raw(format!("Font: {}", self.state.current_name())));
        texts.push(Text::raw(" | "));
        texts.push(Text::raw(format!("Mode: {:?}", self.state.get_render_type())));
        Paragraph::new(texts.iter())
            .block(Block::default().title("info").borders(Borders::ALL))
            .wrap(false)
            .render(f, area);
    }

    fn draw<B>(&self, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(24), Constraint::Length(3)].as_ref())
            .split(f.size());

        let main = layout[0];
        let info = layout[1];

        #[allow(clippy::cast_possible_truncation)] // never truncation because we `min` with 24
        let list_width = self.state.name_width_max().min(24) as u16;

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(list_width), Constraint::Min(24)].as_ref())
            .split(main);

        let list = main[0];
        let canvas = main[1];

        let width = canvas.width.saturating_sub(2);
        let height = canvas.height.saturating_sub(2);
        self.state.update_rect(width, height);

        self.draw_list(list, f);
        self.draw_canvas(canvas, f);
        self.draw_info(info, f);
    }

    #[allow(clippy::unused_self)]
    fn on_event(&mut self, event: CTResult<TerminalEvent>) -> CTResult<OnEventResult> {
        match event? {
            TerminalEvent::Tick => Ok(OnEventResult::Continue),
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
