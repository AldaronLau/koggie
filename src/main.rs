use actix_web::{HttpServer, App as ActixApp, HttpRequest, HttpResponse, web, guard};
use actix_files as fs;
use listenfd::ListenFd;

mod watcher;

/*fn load<'a>(html: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}*/

fn main() {
    println!("Starting 'Koggie v{}'…", env!("CARGO_PKG_VERSION"));

    let ip = std::fs::read_to_string("/usr/share/koggie/ip.txt").unwrap_or_else(
        |_e| {
            eprintln!("Failed to load file!");
            std::process::exit(1);
        }
    );
    let mut ip_port = String::new();
    ip_port.push_str(ip.trim_end());
    ip_port.push_str(":8080");
    println!("Running on {}…", ip_port);

    std::thread::spawn(watcher::watcher);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move ||
        ActixApp::new()
            .service(web::resource("/robots.txt").route(
                web::get().to(|_req: HttpRequest| {
                    HttpResponse::Ok()
                        .content_type("text/plain")
                        .body("User-agent: *\nDisallow: /res/")
                }
            )))
            .service(fs::Files::new("/res", "/usr/share/koggie/other").show_files_listing())
            .service(web::resource("/").route(
                web::get().to(|_req: HttpRequest| {
                    fs::NamedFile::open("/usr/share/koggie/cache/home.html")
                }
            )))
            .service(web::resource("/{page}").route(
                web::get().to(|_req: HttpRequest, path: web::Path<(String,)>| {
                    fs::NamedFile::open(format!("/usr/share/koggie/cache/{}.html", path.0))
                }
            )))
            .default_service(
                web::get().to(|_req: HttpRequest| {
                    fs::NamedFile::open("/usr/share/koggie/cache/404.html")
                })
            )
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(ip_port).unwrap()
    };

    server.run().unwrap();
}
