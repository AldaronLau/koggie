use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const PAGE_TITLE: &str = "KAUG Radio & Concert Promotion";

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

pub fn gen_page(md_file: &str) -> String {
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
