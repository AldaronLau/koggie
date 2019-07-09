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

    println!("Running on IP {}", ip);

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
            .service(fs::Files::new("/", "/usr/share/koggie/cache").index_file("home.html"))
            .default_service(
                web::get().to(|_req: HttpRequest| { fs::NamedFile::open("/usr/share/koggie/cache/404.html") })

/*                web::resource("")
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                ),*/
            )
/*            .service(web::resource("/").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/home.md")) })))
            .service(web::resource("/about").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/about.md")) })))
            .service(web::resource("/artists").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/artists.md")) })))
            .service(web::resource("/calendar").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/calendar.md")) })))
            .service(web::resource("/shows").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/shows.md")) })))
            .service(web::resource("/koggie").route(
                web::get().to(|_req: HttpRequest| { load(gen_page("../site/koggie.md")) })))*/
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(format!("10.0.0.159:8080")).unwrap()
    };

    server.run().unwrap();
}
