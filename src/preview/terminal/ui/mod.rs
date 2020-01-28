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

mod event;
mod state;

use {
    crate::font::SortedFamilies,
    crossterm::{
        event::{KeyCode as CTKeyCode, KeyModifiers as CTKM},
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
        layout::{Constraint, Direction, Layout, Rect},
        terminal::{Frame, Terminal},
        widgets::{canvas::Canvas, Block, Borders, Paragraph, SelectableList, Text, Widget},
    },
};

enum OnEventResult {
    ReDraw,
    Continue,
    Exit,
}

pub struct UI<'a> {
    state: State<'a>,
}

#[allow(clippy::unused_self)] // TODO: use them
impl<'a> UI<'a> {
    pub fn new(families: SortedFamilies<'a>) -> Option<Self> {
        if families.len() > 0 {
            Some(Self { state: State::new(families) })
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
            .items(&self.state.names)
            .select(Some(self.state.index))
            .highlight_symbol(">>")
            .render(f, area);
    }

    fn draw_info<B>(&self, area: Rect, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let mut texts = Vec::new();
        texts.push(Text::raw(self.state.current_name()));
        Paragraph::new(texts.iter()).wrap(false).render(f, area);
    }

    fn draw_canvas<B>(&self, area: Rect, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .paint(|_ctx| {})
            .render(f, area);
    }

    fn draw<B>(&self, f: &mut Frame<B>)
    where
        B: tui::backend::Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(24), Constraint::Length(1)].as_ref())
            .split(f.size());

        let main = layout[0];
        let info = layout[1];

        #[allow(clippy::cast_possible_truncation)] // never truncation because we `min` with 24
        let list_width = self.state.name_max_width.min(24) as u16;

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(list_width), Constraint::Min(24)].as_ref())
            .split(main);

        let list = main[0];
        let canvas = main[1];

        self.draw_list(list, f);
        self.draw_canvas(canvas, f);
        self.draw_info(info, f);
    }

    #[allow(clippy::unused_self)]
    fn on_event(&mut self, event: CTResult<TerminalEvent>) -> CTResult<OnEventResult> {
        match event? {
            TerminalEvent::Tick => Ok(OnEventResult::ReDraw),
            TerminalEvent::Key(key) => {
                if key.modifiers.contains(CTKM::ALT) || key.modifiers.contains(CTKM::CONTROL) {
                    Ok(OnEventResult::Continue)
                } else {
                    match key.code {
                        CTKeyCode::Char('q') => Ok(OnEventResult::Exit),
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
