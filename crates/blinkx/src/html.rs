use std::{error::Error, fmt::Display, fs::read_to_string};

#[derive(Debug)]
pub enum HtmlError {
    NotHtml,
    NoSuchFile(String),
}

impl Display for HtmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotHtml => write!(f, "This file does not contain valid html."),
            Self::NoSuchFile(path) => write!(f, "File {path} doesn't exist."),
        }
    }
}

impl Error for HtmlError {}

fn is_html(file_path: &str) -> bool {
    file_path.ends_with(".html")
}

pub fn dom_from_file(file_path: &str) -> Result<html_parser::Dom, HtmlError> {
    if !is_html(file_path) {
        return Err(HtmlError::NotHtml);
    }

    let Ok(open_file) = read_to_string(file_path) else {
        return Err(HtmlError::NoSuchFile(file_path.into()));
    };

    html_parser::Dom::parse(&open_file).map_err(|_| HtmlError::NotHtml)
}

