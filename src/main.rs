use actix_web::{server, App as ActixApp, HttpRequest, Responder};
use listenfd::ListenFd;

//static HEAD: &'static str = include!("");
//static FOOT: &'static str = include_bytes!("");

fn index(_req: &HttpRequest) -> impl Responder {
    "Hello World!"
}

fn main() {
    println!("Starting KOGGIE…");

    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 3);

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        ActixApp::new()
            .resource("/", |r| r.f(index))
            .resource("/about", |r| r.f(index))
            .resource("/artists", |r| r.f(index))
            .resource("/calendar", |r| r.f(index))
            .resource("/shows", |r| r.f(index))
            .resource("/koggie", |r| r.f(index))
            .default_resource(|r| r.f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(format!("{}:8080", args[2])).unwrap()
    };

    server.run();
}
