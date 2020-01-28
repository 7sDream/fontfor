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

use {
    crossterm::{
        event::{self, Event as CTEvent, KeyEvent as CTKeyEvent},
        Result as CTResult,
    },
    std::{ops::Deref, sync::mpsc, thread, time::Duration},
};

#[derive(Copy, Clone, Debug, PartialOrd, Eq, PartialEq, Hash)]
pub enum TerminalEvent {
    Tick,
    Key(CTKeyEvent),
}

pub struct TerminalEventStream {
    rx: mpsc::Receiver<CTResult<TerminalEvent>>,
}

impl TerminalEventStream {
    pub fn new(tick_interval: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        let keyboard_tx = tx;
        thread::spawn(move || keyboard_event_generator(tick_interval, keyboard_tx));

        Self { rx }
    }
}

impl Deref for TerminalEventStream {
    type Target = mpsc::Receiver<CTResult<TerminalEvent>>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

#[allow(clippy::needless_pass_by_value)] // because it is run in other thread
fn keyboard_event_generator(tick_interval: Duration, tx: mpsc::Sender<CTResult<TerminalEvent>>) {
    loop {
        match event::poll(tick_interval) {
            Ok(true) => {
                if let CTEvent::Key(key) = event::read().unwrap() {
                    if tx.send(Ok(TerminalEvent::Key(key))).is_err() {
                        break;
                    }
                }
            }
            Ok(false) => {
                if tx.send(Ok(TerminalEvent::Tick)).is_err() {
                    break;
                }
            }
            Err(kind) => {
                if tx.send(Err(kind)).is_err() {
                    break;
                }
            }
        }
    }
}
