use std::{collections::HashMap, str::Lines};


pub struct Metadata<'a> {
    pub language: &'a str,
    pub charset: &'a str,
    pub title: &'a str,

    pub author: Option<&'a str>,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,

    pub stylesheets: Vec<&'a str>,
    pub templates: Vec<&'a str>,
    pub other: HashMap<&'a str, &'a str>
}

impl<'a> Default for Metadata<'a> {
    fn default() -> Self {
        Metadata {
            language: "en",
            charset: "UTF-8",
            title: "",

            author: None,
            description: None,
            icon: None,

            stylesheets: vec![],
            templates: vec![],
            other: HashMap::new()
        }
    }
}


// Extract metadata from a Lines iterator, leaving all non-metadata lines behind
pub fn extract<'a>(lines: Lines<'a>) -> (Metadata<'a>, String) {
    let mut meta = Metadata::default();

    let lines = lines.filter(|line| {
        if line.starts_with("$[") && line.ends_with(")]") {
            let (key, val) = line[2..(line.len() - 2)].split_once('(').unwrap();
            let val = val.trim_matches(|ch: char| ch.is_whitespace() || ch == '"' || ch == '\'');

            match key {
                "language" | "lang" => meta.language = val,
                "charset" => meta.charset = val,
                "title" => meta.title = val,

                "author" => meta.author = Some(val),
                "description" | "desc" => meta.description = Some(val),
                "icon" => meta.icon = Some(val),

                "stylesheet" | "style" => {
                    meta.stylesheets.push(val);
                },

                "template" => {
                    meta.templates.push(val);
                },

                _ => {
                    meta.other.insert(key, val);
                }
            }

            return false;
        }

        true
    });

    let without_meta = lines.collect::<Vec<&str>>().join("\n");

    (meta, without_meta)
}
