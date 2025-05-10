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

use std::{
    cell::RefCell,
    io::{Error as IOError, ErrorKind as IOErrorKind, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{Receiver, Sender, TryRecvError, channel},
    thread,
    time::Duration,
};

use httparse::Request;

pub struct SingleThread {
    html: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum RequestStatus {
    WrongMethod,
    WrongPath,
    NotHttp,
    Good,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum CheckRequestResult {
    Complete,
    Part(RequestStatus),
    Unknown,
}

impl SingleThread {
    pub fn new(html: String) -> Self {
        Self { html }
    }

    fn response_status_line(code: u16, reason: &str) -> String {
        format!("HTTP/1.1 {code} {reason}\r\n")
    }

    fn response_common(code: u16, reason: &str, headers: &[&str]) -> String {
        format!(
            "{}{}Connection: close\r\n\r\n",
            Self::response_status_line(code, reason),
            headers.join("")
        )
    }

    fn response_200(content: &str) -> String {
        let content_length_header = format!("Content-Length: {}\r\n", content.len());
        let headers = [
            "Content-Type: text/html\r\n",
            "Content-Encoding: utf-8\r\n",
            &content_length_header,
        ];

        Self::response_common(200, "OK", &headers)
    }

    fn response_400() -> String {
        Self::response_common(400, "Bad Request", &[])
    }

    fn response_404() -> String {
        Self::response_common(404, "Not Found", &[])
    }

    fn response_405() -> String {
        Self::response_common(405, "Method Not Allowed", &[])
    }

    fn check_req(buffer: &[u8], last: CheckRequestResult) -> CheckRequestResult {
        if last == CheckRequestResult::Complete {
            return last;
        }

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = Request::new(&mut headers);
        match req.parse(buffer) {
            Ok(result) => {
                // We Only accept `GET /` request
                if last == CheckRequestResult::Unknown {
                    if let Some(method) = req.method {
                        if method != "GET" {
                            return CheckRequestResult::Part(RequestStatus::WrongMethod);
                        }
                    } else {
                        return CheckRequestResult::Unknown;
                    }
                    if let Some(path) = req.path {
                        if path != "/" {
                            return CheckRequestResult::Part(RequestStatus::WrongPath);
                        }
                    } else {
                        return CheckRequestResult::Unknown;
                    }
                }
                if result.is_complete() {
                    CheckRequestResult::Complete
                } else {
                    CheckRequestResult::Part(RequestStatus::Good)
                }
            }
            Err(_) => CheckRequestResult::Part(RequestStatus::NotHttp),
        }
    }

    fn handler(
        mut stream: TcpStream, response_body: &str, exit_rx: &Receiver<()>,
    ) -> Result<bool, IOError> {
        const TRY_PARSE_SIZE: usize = 1024;
        let buffer = RefCell::new([0_u8; TRY_PARSE_SIZE]);
        let mut offset = 0;
        let mut status = CheckRequestResult::Unknown;

        loop {
            let next_offset = offset
                + match stream.read(&mut buffer.borrow_mut()[offset..]) {
                    Ok(size) => size,
                    Err(err) => {
                        if err.kind() == IOErrorKind::WouldBlock {
                            thread::sleep(Duration::from_millis(100));
                            match exit_rx.try_recv() {
                                Ok(_) | Err(TryRecvError::Disconnected) => return Ok(false),
                                _ => continue,
                            }
                        } else {
                            return Err(err);
                        }
                    }
                };

            if next_offset == TRY_PARSE_SIZE {
                // Too big as a simple `GET /` request, don't handle it
                stream.write_all(Self::response_400().as_bytes())?;
                return Ok(true);
            }

            let req_content = buffer.borrow();

            status = Self::check_req(&req_content[0..next_offset], status);

            match status {
                CheckRequestResult::Complete => {
                    stream.write_all(Self::response_200(response_body).as_bytes())?;
                    stream.write_all(response_body.as_bytes())?;
                    return Ok(true);
                }
                CheckRequestResult::Part(RequestStatus::WrongMethod) => {
                    stream.write_all(Self::response_405().as_bytes())?;
                    return Ok(true);
                }
                CheckRequestResult::Part(RequestStatus::WrongPath) => {
                    stream.write_all(Self::response_404().as_bytes())?;
                    return Ok(true);
                }
                CheckRequestResult::Part(RequestStatus::NotHttp) => {
                    stream.write_all(Self::response_400().as_bytes())?;
                    return Ok(true);
                }
                _ => (),
            }

            offset = next_offset;
        }
    }

    fn server(
        addr_tx: Sender<SocketAddr>, exit_rx: Receiver<()>, content: String,
    ) -> Result<(), IOError> {
        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        let server = match TcpListener::bind((loopback, 0)) {
            Ok(server) => server,
            Err(err) => {
                eprintln!("Error when start http server: {err:?}");
                std::process::exit(-1)
            }
        };

        let addr = server
            .local_addr()
            .expect("tcp listener must have a local addr");
        if addr_tx.send(addr).is_err() {
            return Ok(());
        }

        // set non-blocking mode to give chance to receive exit message
        server.set_nonblocking(true)?;

        for stream in server.incoming() {
            match stream {
                Ok(stream) => match Self::handler(stream, &content, &exit_rx) {
                    Ok(true) => {}
                    Ok(false) => break,
                    Err(err) => {
                        eprintln!("Error when process request: {err:?}");
                    }
                },
                Err(err) if err.kind() == IOErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100));
                }
                Err(err) => {
                    eprintln!("Error when listening: {err:?}");
                }
            }

            match exit_rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }
        }

        Ok(())
    }

    pub fn run_until<F>(self, stop: F)
    where
        F: FnOnce(SocketAddr),
    {
        let (addr_tx, addr_rx) = channel();
        let (exit_tx, exit_rx) = channel();

        let handler = thread::spawn(|| Self::server(addr_tx, exit_rx, self.html));

        if let Ok(addr) = addr_rx.recv() {
            stop(addr);
        }

        exit_tx.send(()).expect("server hold exit_rx forever");

        handler.join().unwrap().unwrap();
    }
}
