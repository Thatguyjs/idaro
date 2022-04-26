// Construct an HTML document from metadata and a body

use super::metadata::Metadata;


macro_rules! meta_tag {
    ($name:expr, $content:expr) => {
        format!(r#"<meta name="{}" content="{}">"#, $name, $content)
    }
}

macro_rules! link_tag {
    ($rel:expr, $path:expr) => {
        format!(r#"<link rel="{}" href="{}">"#, $rel, $path)
    }
}


pub fn format_metadata(metadata: &Metadata) -> String {
    let mut result = vec![
        format!("<meta charset=\"{}\">", metadata.charset)
    ];

    if let Some(author) = metadata.author {
        result.push(meta_tag!("author", author));
    }
    if let Some(desc) = metadata.description {
        result.push(meta_tag!("description", desc));
    }
    if let Some(icon) = metadata.icon {
        result.push(link_tag!("icon", icon));
    }

    for (name, content) in &metadata.other {
        result.push(meta_tag!(name, content));
    }

    result.push(format!("<title>{}</title>", metadata.title));

    for path in &metadata.stylesheets {
        result.push(link_tag!("stylesheet", path));
    }

    if metadata.templates.len() > 0 {
        println!("WARNING: Markdown templates have not been introduced yet!");
    }

    result.join("\n        ")
}

pub fn format(metadata: Metadata, body: String) -> String {
    format!(
r#"<!DOCTYPE html>
<html lang="{}">
    <head>
        {}
    </head>
    <body>
        {}
    </body>
</html>"#,

        metadata.language,
        format_metadata(&metadata),
        body.trim()
    )
}
