use actix_web::{server, App as ActixApp, fs, HttpRequest, HttpResponse, Responder, http::Method};
use listenfd::ListenFd;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const PAGE_TITLE: &str = "KAUG Radio & Concert Promotion";

fn load<'a>(html: String) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

fn parse_mdline(input: &str, output: &mut String, paragraph_last: &mut bool) {
    if input.len() <= 1 {
        if *paragraph_last {
            output.push_str("</p>");
            *paragraph_last = false;
        }
    } else if input.starts_with("#") {
        let input2 = input.trim_start_matches("#");
        if *paragraph_last {
            output.push_str("</p>");
            *paragraph_last = false;
        }
        output.push_str(&format!("<h{}>", input.len() - input2.len()));
        output.push_str(input2);
        output.push_str(&format!("</h{}>", input.len() - input2.len()));
        if input.len() - input2.len() == 1 {
            output.push_str("<hr/>");
        }
    } else {
        if *paragraph_last == false {
            output.push_str("<p>");
            *paragraph_last = true;
        }
        if input.starts_with("-") {
            let input2 = input.trim_start_matches("-");
            output.push_str("<ul><li>");
            output.push_str(input2);
            output.push_str("</li></ul>");
        } else if input.starts_with("*") {
            let input2 = input.trim_start_matches("*");
            output.push_str("<ul><li>");
            output.push_str(input2);
            output.push_str("</li></ul>");
        } else {
            output.push_str(input);
        }
    }
}

fn gen_page(md_file: &str) -> String {
    let mut paragraph_last = false;
    let mut gen_content = "".to_string();
    let md_content = if let Ok(mut file) = File::open(md_file) {
        let mut contents = "".to_string();
        let _ = file.read_to_string(&mut contents);
        contents
    } else {
        "".to_string()
    };
    let mut lines = md_content.lines();
    let first = if let Some(line) = lines.next() {
        line
    } else {
        ""
    };
    let mut page_title = None;
    let possible_title = first.to_string();
    if first.starts_with("#") {
        page_title = Some(possible_title.trim_start_matches('#'));
    }
    parse_mdline(first, &mut gen_content, &mut paragraph_last);
    for line in lines {
        parse_mdline(line, &mut gen_content, &mut paragraph_last);
    }

    let mut page = include!("../res/html.rs").to_string();
    if let Some(title) = page_title {
        page.push_str(title);
        page.push_str(" | ");
    }
    page.push_str(PAGE_TITLE);
    page.push_str(concat!(
        include!("../res/head.rs"),
        include!("../res/menu.rs")
    ));
    page.push_str(&gen_content);
    page.push_str(include!("../res/foot.rs"));

    page
}

fn main() {
    println!("Starting 'KOGGIE v{}'……", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 3);

    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(move || {
        ActixApp::new()
            .resource("/", |r| r.f(|_req| {
                load(gen_page("../site/home.md"))
            }))
            .resource("/about", |r| r.f(|_req| {
                load(gen_page("../site/about.md"))
            }))
            .resource("/artists", |r| r.f(|_req| {
                load(gen_page("../site/artists.md"))
            }))
            .resource("/calendar", |r| r.f(|_req| {
                load(gen_page("../site/calendar.md"))
            }))
            .resource("/shows", |r| r.f(|_req| {
                load(gen_page("../site/shows.md"))
            }))
            .resource("/koggie", |r| r.f(|_req| {
                load(gen_page("../site/koggie.md"))
            }))
            .resource("/robots.txt", |r| {r.f(|_req| {
                HttpResponse::Ok()
                    .content_type("text/plain")
                    .body("User-agent: *\nDisallow: /res/")
            }
            )})
            .handler("/res", fs::StaticFiles::new("../res").unwrap())
            .default_resource(|r| r.f(|_req| {
                load(gen_page("../site/404.md"))
            }))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind(format!("{}:8080", args[2])).unwrap()
    };

    server.run();
}
