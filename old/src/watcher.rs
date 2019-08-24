use pulldown_cmark::{Parser, Options, html};
use inotify::{
    EventMask,
    WatchMask,
    Inotify,
};

macro_rules! prefix {
    () => ("/usr/share/koggie/")
}

struct Context {
    page_title: String,
    head: String,
    body: String,
    foot: String,
}

fn add_links(input: &str) -> String {
    let mut output = String::new();
    for line in input.lines() {
        let words = line.split(' ');
        for word in words {
            let is_email = word.contains('@') && word.contains('.');
            let is_link = word.starts_with("http://") || is_email;
            if is_link {
                output.push('[');
            }
            output.push_str(word);
            if is_link {
                output.push_str("](");
                output.push_str(if is_email {
                    "mailto:"
                } else {
                    "https://"
                });
                output.push_str(word);
                output.push(')');
            }
            output.push(' ');
        }
        output.push('\n');
    }
    output
}

fn gen_page(context: &Context, markdown_filename: &str) {
    let markdown_filename = std::path::Path::new(markdown_filename);

    println!("Generating Page {}…", markdown_filename.display());

    let markdown_input = &if let Ok(input) = std::fs::read_to_string(markdown_filename) {
        input
    } else {
        eprintln!("Couldn't read file '{}'?", markdown_filename.display());
        return;
    };
    let markdown_input = &add_links(markdown_input);
    let parser = Parser::new_ext(markdown_input, Options::all());
     
    let mut page = r#"<!DOCTYPE html><html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>"#.to_string();
    if markdown_input.starts_with('#') {
        if let Some(offset) = markdown_input.find('\n') {
            let title = markdown_input.split_at(offset).0[1..].trim_start();
            page.push_str(title);
            page.push_str(" | ");
        }
    }
    page.push_str(&context.page_title);
    page.push_str("</title>");
    page.push_str(&context.head);
    page.push_str("</head><body>");
    page.push_str(&context.body);
    html::push_html(&mut page, parser);
    page.push_str(&context.foot);

    if let Some(stem) = markdown_filename.file_stem() {
        let mut path = "/usr/share/koggie/cache/".to_string();
        path.push_str(&stem.to_string_lossy());
        path.push_str(".html");
        let _ = std::fs::write(path, page);
    } else {
        eprintln!("ERROR: No available stem!");
    }

    println!("Generated Page {}…", markdown_filename.display());
}

// generate all HTML files from markdown.
fn gen_all(context: &Context) {
    let iter = if let Ok(it) = std::fs::read_dir("/usr/share/koggie/pages") {
        it
    } else {
        eprintln!("/usr/share/koggie/pages/ is missing!");
        std::process::exit(1);
    };

    for dir in iter {
        let entry = match dir {
            Ok(dir) => { dir }
            Err(e) => {
                eprintln!("Fail {:?}", e);
                continue;
            },
        };

        gen_page(context, &entry.path().as_path().to_string_lossy());
    }
}

pub fn watcher() {
    let context = Context {
        page_title: std::fs::read_to_string(concat!(prefix!(), "names/title.txt")).unwrap_or_else(|_| {
            eprintln!("Required file /names/title.txt not found.");
            std::process::exit(1)
        }),
        head: std::fs::read_to_string(concat!(prefix!(), "style/head.html")).unwrap_or_else(|_| {
            eprintln!("Required file /style/head.html not found.");
            std::process::exit(1)
        }),
        body: std::fs::read_to_string(concat!(prefix!(), "style/body.html")).unwrap_or_else(|_| {
            eprintln!("Required file /style/body.html not found.");
            std::process::exit(1)
        }),
        foot: std::fs::read_to_string(concat!(prefix!(), "style/foot.html")).unwrap_or_else(|_| {
            eprintln!("Required file /style/foot.html not found.");
            std::process::exit(1)
        }),
    };

    gen_all(&context);

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify.add_watch(concat!(prefix!(), "pages"), WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE).expect("Failed to add inotify watch");
    inotify.add_watch(concat!(prefix!(), "style"), WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE).expect("Failed to add inotify watch");
    inotify.add_watch(concat!(prefix!(), "names/title.txt"), WatchMask::MODIFY | WatchMask::CREATE | WatchMask::DELETE).expect("Failed to add inotify watch");

    println!("Watching current directory for activity...");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        'e: for event in events {
            let event_name = if let Some(event_name) = event.name {
                event_name
            } else {
                continue 'e;
            };
            let event_name = event_name.to_string_lossy();

            if event_name.ends_with(".md") {
                if event.mask.contains(EventMask::MODIFY) {
                    if !event.mask.contains(EventMask::ISDIR) {
                        let mut file = prefix!().to_string();
                        file.push_str(&event_name);
                        println!("File update: {}", event_name);
                        gen_page(&context, &file);
                    }
                } else if event.mask.contains(EventMask::DELETE) {
                    if !event.mask.contains(EventMask::ISDIR) {
                        let file: &str = &event_name;
                        let file = std::path::Path::new(file);
                        let stem = if let Some(stem) = file.file_stem() {
                            stem
                        } else {
                            eprintln!("Couldn't get file stem!");
                            continue 'e;
                        };
                        let mut file = "/usr/share/koggie/cache/".to_string();
                        file.push_str(&stem.to_string_lossy());
                        file.push_str(".html");
                        println!("File delete: {:?}", event_name);
                        let _ = std::fs::remove_file(file);
                    }
                }
            } else if event_name.ends_with(".html") || event_name == "title.txt" {
                if event.mask.contains(EventMask::MODIFY) {
                    if !event.mask.contains(EventMask::ISDIR) {
                        gen_all(&context);
                    }
                }
            }
        }

        println!("Building website…");
    }
}
