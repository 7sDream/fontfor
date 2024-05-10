// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 - 2023 7sDream <i@7sdre.am> and contributors
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

mod cache;
mod event;
mod state;

use std::{
    io::{Result as IoResult, Stdout},
    time::Duration,
};

use crossterm::{
    event::{Event, KeyCode as CtKeyCode, KeyEvent, KeyModifiers as CtKM},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::{Frame, Terminal},
    text::{Line, Span, Text},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, Paragraph},
};
use tui_input::backend::crossterm::EventHandler;

use self::{
    cache::{GlyphCache, GlyphCanvasShape},
    event::{TerminalEvent, TerminalEventStream},
    state::State,
};
use crate::family::FilteredFamilies;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum OnEventResult {
    ReDraw,
    Continue,
    Exit,
}

enum WhichInput {
    Search,
}

pub struct UI<'a> {
    idle_redraw: u8,
    filter_input: tui_input::Input,
    editing: Option<WhichInput>,
    state: State<'a>,
}

impl<'a> UI<'a> {
    pub fn new(filtered: FilteredFamilies<'a>) -> Option<Self> {
        if !filtered.is_empty() {
            Some(Self {
                idle_redraw: 0,
                filter_input: tui_input::Input::new(filtered.keyword().to_string()),
                editing: None,
                state: State::new(filtered),
            })
        } else {
            None
        }
    }

    fn draw_list(&self, area: Rect, f: &mut Frame<'_>) {
        let families = self.state.font_face_names();
        let index = self.state.index();
        let title = format!(
            "Fonts {}/{}",
            index.map(|x| x + 1).unwrap_or_default(),
            self.state.len()
        );

        let list = List::new(families.map(ListItem::new).collect::<Vec<_>>())
            .block(
                Block::default()
                    .title(Span::raw(title))
                    .borders(Borders::ALL),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(list, area, &mut self.state.mut_list_state())
    }

    fn draw_filter_input(&self, area: Rect, f: &mut Frame<'_>) {
        let searchbox = &self.filter_input;
        let scroll = searchbox.visual_scroll(area.width as usize - 3);

        let style = if let Some(WhichInput::Search) = self.editing {
            let x = searchbox.visual_cursor().max(scroll) - scroll;
            f.set_cursor(area.x + 1 + x as u16, area.y + 1);

            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let input = Paragraph::new(searchbox.value())
            .scroll((0, scroll as u16))
            .style(style)
            .block(Block::default().borders(Borders::ALL).title("Filter"));

        f.render_widget(input, area);
    }

    fn draw_preview_canvas(&self, area: Rect, f: &mut Frame<'_>, shape: &GlyphCanvasShape) {
        let (canvas_width, canvas_height) = self.state.get_canvas_size_by_pixel();
        let canvas_width = f64::from(canvas_width);
        let canvas_height = f64::from(canvas_height);
        let canvas = Canvas::default()
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .x_bounds([0.0, canvas_width])
            .y_bounds([0.0, canvas_height])
            .paint(|ctx| {
                ctx.draw(shape);
            });
        f.render_widget(canvas, area);
    }

    fn draw_preview_paragraph<'s, I>(&self, area: Rect, f: &mut Frame<'_>, paragraph: I)
    where
        I: IntoIterator<Item = &'s str>,
        I::IntoIter: ExactSizeIterator,
    {
        let (_, height) = self.state.get_canvas_size_by_char();

        let iter = paragraph.into_iter();

        // saturating_sub here makes padding zero instead of overflow to a huge number
        // if render result taller then preview area
        let padding = (height as usize).saturating_sub(iter.len());
        let mut lines = vec![Line::from(""); padding / 2];

        for line in iter {
            lines.push(Line::from(line));
        }

        let canvas = Paragraph::new(Text::from(lines))
            .block(Block::default().title("Preview").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(
                Style::default()
                    .fg(Color::Reset)
                    .add_modifier(Modifier::BOLD),
            );
        f.render_widget(canvas, area);
    }

    fn draw_preview(&self, area: Rect, f: &mut Frame<'_>) {
        let result = self.state.render();

        if let Some(result) = result {
            match result.as_ref().as_ref() {
                Ok(GlyphCache::Canvas(ref shape)) => {
                    self.draw_preview_canvas(area, f, shape);
                }
                Ok(GlyphCache::Paragraph(ref s)) => {
                    self.draw_preview_paragraph(area, f, s.lines.iter().map(|s| s.as_str()))
                }
                Err(s) => {
                    let paragraph: [&str; 1] = [s];
                    self.draw_preview_paragraph(area, f, paragraph)
                }
            }
        } else {
            self.draw_preview_paragraph(area, f, [])
        }
    }

    fn generate_help_text<'x>(key: &'x str, help: &'x str) -> Vec<Span<'x>> {
        vec![
            Span::styled(
                key,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(": "),
            Span::styled(
                help,
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ]
    }

    fn draw_status_bar_info(&self, area: Rect, f: &mut Frame<'_>) {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
            .split(area);

        let name = cols[0];
        let mode = cols[1];

        let texts = vec![
            Span::styled("Font Face", Style::default().fg(Color::Green)),
            Span::raw(": "),
            Span::styled(
                self.state.current_name().unwrap_or_default(),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ];
        f.render_widget(
            Paragraph::new(Line::from(texts))
                .block(
                    Block::default()
                        .title("Info")
                        .borders(Borders::TOP | Borders::LEFT),
                )
                .alignment(Alignment::Left),
            name,
        );

        let texts = vec![
            Span::styled("Render", Style::default().fg(Color::Green)),
            Span::raw(": "),
            Span::styled(
                format!("{:?}", self.state.get_render_type()),
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        ];
        f.render_widget(
            Paragraph::new(Line::from(texts))
                .block(Block::default().borders(Borders::TOP | Borders::RIGHT))
                .alignment(Alignment::Right),
            mode,
        );
    }

    fn draw_status_bar_help(&self, area: Rect, f: &mut Frame<'_>) {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .split(area);

        let filter_help = if self.editing.is_none() {
            Self::generate_help_text("[S/Slash]", "Filter Box")
        } else {
            Self::generate_help_text("[ESC/Enter]", "Exit Edit")
        };
        let list_help = Self::generate_help_text("[Up/Down]", "Select Font");
        let mode_help = Self::generate_help_text(
            "[Left/Right]",
            if self.editing.is_none() {
                "Change Render"
            } else {
                "Move Cursor"
            },
        );
        let quit_help = if self.editing.is_none() {
            Self::generate_help_text("[Q]", "Quit")
        } else {
            filter_help.clone()
        };

        f.render_widget(
            Paragraph::new(Line::from(filter_help))
                .block(Block::default().borders(Borders::BOTTOM | Borders::LEFT))
                .alignment(Alignment::Left),
            cols[0],
        );

        f.render_widget(
            Paragraph::new(Line::from(list_help))
                .block(Block::default().borders(Borders::BOTTOM))
                .alignment(Alignment::Center),
            cols[1],
        );

        f.render_widget(
            Paragraph::new(Line::from(mode_help))
                .block(Block::default().borders(Borders::BOTTOM))
                .alignment(Alignment::Center),
            cols[2],
        );

        f.render_widget(
            Paragraph::new(Line::from(quit_help))
                .block(Block::default().borders(Borders::BOTTOM | Borders::RIGHT))
                .alignment(Alignment::Right),
            cols[3],
        );
    }

    fn draw_status_bar(&self, area: Rect, f: &mut Frame<'_>) {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Length(2)].as_ref())
            .split(area);

        let info = rows[0];
        let help = rows[1];

        self.draw_status_bar_info(info, f);
        self.draw_status_bar_help(help, f);
    }

    fn draw(&self, f: &mut Frame<'_>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(4)].as_ref())
            .split(f.size());

        let main = layout[0];
        let status_bar = layout[1];

        let list_width = self.state.name_width_max().clamp(24, 64) as u16;

        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(list_width), Constraint::Min(24)].as_ref())
            .split(main);

        let side_pannel = main[0];
        let canvas = main[1];

        let side_pannel = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .split(side_pannel);

        let list = side_pannel[0];
        let searchbox = side_pannel[1];

        let width = u32::from(canvas.width.saturating_sub(2));
        let height = u32::from(canvas.height.saturating_sub(2));
        self.state.update_canvas_size_by_char(width, height);

        self.draw_list(list, f);
        self.draw_filter_input(searchbox, f);
        self.draw_preview(canvas, f);
        self.draw_status_bar(status_bar, f);
    }

    fn on_event_edit_filter(&mut self, key: KeyEvent) -> OnEventResult {
        if matches!(key.code, CtKeyCode::Enter | CtKeyCode::Esc) {
            self.editing = None;
            return OnEventResult::ReDraw;
        }

        if let Some(change) = self.filter_input.handle_event(&Event::Key(key)) {
            if change.value {
                self.state
                    .update_search_box(Some(self.filter_input.value()));
            }
            OnEventResult::ReDraw
        } else {
            OnEventResult::Continue
        }
    }

    fn on_event_normal(&mut self, key: KeyEvent) -> OnEventResult {
        if key.modifiers.contains(CtKM::ALT) || key.modifiers.contains(CtKM::CONTROL) {
            OnEventResult::Continue
        } else {
            match key.code {
                CtKeyCode::Char('q') => OnEventResult::Exit,
                CtKeyCode::Left | CtKeyCode::Char('h') => {
                    self.state.prev_render_type();
                    OnEventResult::ReDraw
                }
                CtKeyCode::Right | CtKeyCode::Char('l') => {
                    self.state.next_render_type();
                    OnEventResult::ReDraw
                }
                CtKeyCode::Char('j') => {
                    self.state.move_down();
                    OnEventResult::ReDraw
                }
                CtKeyCode::Char('k') => {
                    self.state.move_up();
                    OnEventResult::ReDraw
                }
                CtKeyCode::Char('s') | CtKeyCode::Char('/') => {
                    self.editing = Some(WhichInput::Search);
                    OnEventResult::ReDraw
                }
                _ => OnEventResult::Continue,
            }
        }
    }

    fn on_event(&mut self, event: IoResult<TerminalEvent>) -> IoResult<OnEventResult> {
        match event? {
            TerminalEvent::Tick => {
                self.idle_redraw += 1;
                Ok(if self.idle_redraw == 10 {
                    OnEventResult::ReDraw
                } else {
                    OnEventResult::Continue
                })
            }
            TerminalEvent::Key(key) => match key.code {
                CtKeyCode::Up => {
                    self.state.move_up();
                    Ok(OnEventResult::ReDraw)
                }
                CtKeyCode::Down => {
                    self.state.move_down();
                    Ok(OnEventResult::ReDraw)
                }
                _ => match self.editing {
                    Some(WhichInput::Search) => Ok(self.on_event_edit_filter(key)),
                    None => Ok(self.on_event_normal(key)),
                },
            },
        }
    }

    fn setup() -> IoResult<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(terminal)
    }

    fn shutdown(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> IoResult<()> {
        terminal.show_cursor()?;
        let backend = terminal.backend_mut();
        execute!(backend, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn show(mut self) -> IoResult<()> {
        let mut terminal = Self::setup()?;

        let events = TerminalEventStream::new(Duration::from_millis(1000 / 60));

        terminal.draw(|f| self.draw(f))?;

        for event in events.iter() {
            match self.on_event(event) {
                Err(kind) => {
                    Self::shutdown(terminal)?;
                    return Err(kind);
                }
                Ok(result) => match result {
                    OnEventResult::ReDraw => {
                        terminal.draw(|f| self.draw(f))?;
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
