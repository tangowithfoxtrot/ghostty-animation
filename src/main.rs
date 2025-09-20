use include_dir::include_dir;
use regex::Regex;
use std::{
    io::{stdout, BufWriter, Write},
    time::Duration,
};

const SOURCE_DIR: include_dir::Dir<'_> = include_dir!("./res/");

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset, // no color
}

impl From<String> for Color {
    fn from(color: String) -> Self {
        match color.to_lowercase() {
            val if val == "red" => Color::Red,
            val if val == "green" => Color::Green,
            val if val == "blue" => Color::Blue,
            val if val == "yellow" => Color::Yellow,
            val if val == "magenta" => Color::Magenta,
            val if val == "cyan" => Color::Cyan,
            val if val == "white" => Color::White,
            _ => Color::Reset,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ansi_escape_color = match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
        };
        write!(f, "{}", ansi_escape_color)
    }
}

trait Colorize {
    fn color_content(&self, color: &Color, re: &Regex) -> String;
}

impl Colorize for &str {
    fn color_content(&self, color: &Color, re: &Regex) -> String {
        let ansi_escape_color = match color {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Reset => "\x1b[0m",
        };

        let result: std::borrow::Cow<'_, str> =
            re.replace_all(self, |captures: &regex::Captures| {
                format!("{}{}{}", ansi_escape_color, &captures[1], Color::Reset)
            });
        result.into_owned()
    }
}

impl Colorize for &[u8] {
    fn color_content(&self, color: &Color, re: &Regex) -> String {
        let input_str = std::str::from_utf8(self).unwrap_or("");
        input_str.color_content(color, re)
    }
}

fn main() {
    let color_arg = if !std::env::var("NO_COLOR").unwrap_or_default().is_empty()
        && std::env::args().nth(1).unwrap_or_default().is_empty()
    {
        "unset".to_owned()
    } else {
        std::env::args().nth(1).unwrap_or("blue".to_owned())
    };

    let argv2 = std::env::args()
        .nth(2)
        .unwrap_or("40".to_owned())
        .parse()
        .unwrap_or(40);
    let dur = Duration::from_millis(argv2);

    let ghostty = SOURCE_DIR;

    let stdout = stdout();
    let mut buf_writer = BufWriter::new(stdout);

    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl+C handler");

    let html_element = Regex::new(r#"<span class="b">(.*?)</span>"#).unwrap();
    let color = Color::from(color_arg.clone());

    let mut files = vec![];
    for ghost in ghostty.files() {
        let file = ghost.contents().color_content(&color, &html_element);
        files.push(file);
    }

    loop {
        for file in &files {
            let _ = write!(&mut buf_writer, "{file}");
            std::thread::sleep(dur);
        }
    }
}
