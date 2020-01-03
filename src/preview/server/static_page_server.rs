// FontFor: find fonts which can show a specified character
// Copyright (C) 2019 7sDream <i@7sdre.am> and contributors
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
    hyper::{
        http::Result as HttpResult,
        service::{make_service_fn, service_fn},
        Body, Method, Request, Response, Server,
    },
    std::{convert::Infallible, net::SocketAddr, sync::Arc},
    tokio::runtime,
};

pub struct StaticPageServer {
    html: Option<String>,
}

impl StaticPageServer {
    async fn handler(html: Arc<String>, req: Request<Body>) -> HttpResult<Response<Body>> {
        if req.method() == Method::GET && req.uri().path() == "/" {
            let body: String = String::from(html.as_str());
            Ok(Response::new(body.into()))
        } else {
            Ok(Response::builder().status(404).body("".into()).unwrap())
        }
    }

    pub fn new(html: String) -> Self {
        Self { html: Some(html) }
    }

    pub fn run_until<F>(self, stop: F)
    where
        F: FnOnce(SocketAddr) + Send + 'static,
    {
        let html = Arc::new(self.html.unwrap());
        let make_svc = make_service_fn(move |_conn| {
            let html = html.clone();
            async {
                let f = service_fn(move |req| Self::handler(html.clone(), req));
                Ok::<_, Infallible>(f)
            }
        });

        let mut rt = runtime::Builder::new().basic_scheduler().enable_all().build().unwrap();

        rt.block_on(async {
            let addr = SocketAddr::from(([127, 0, 0, 1], 0));
            let server = Server::bind(&addr).serve(make_svc);
            let addr = server.local_addr();
            tokio::spawn(server);
            tokio::task::spawn_blocking(move || stop(addr)).await.unwrap();
        });
    }
}
