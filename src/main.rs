use actix_web::{server, App as ActixApp, HttpRequest, Responder};
use listenfd::ListenFd;

fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!"
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        ActixApp::new().resource("/", |r| r.f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(format!("{}:80", args[1])).unwrap()
    };

    server.run();
}
