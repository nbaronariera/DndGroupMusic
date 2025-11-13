use id3::{TagLike, Version};
use pulldown_cmark::{Event, Parser, Tag};
use regex::Regex;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() -> anyhow::Result<()> {
    let markdown = fs::read_to_string("./file.md")?;

    let parser = Parser::new(&markdown);

    let mut current_path = vec!["./music".to_owned()];
    let mut current_heading_level = None;

    let re_list = Regex::new(
        r#"(?x)
        (https?://[^\s,]+)     # grupo 1: URL (hasta la primera coma o espacio)
        \s*,\s*                # separador coma con posibles espacios
        (.+)                   # grupo 2: todo lo demás (las etiquetas)
        "#,
    )?;

    for event in parser {
        match event {
            Event::Start(Tag::Heading {
                level,
                id: _,
                classes: _,
                attrs: _,
            }) => {
                // Marca que empieza un título
                current_heading_level = Some(level);
            }
            Event::Text(text) => {
                // Si justo veníamos de un heading, este texto es su contenido
                if let Some(level) = current_heading_level.take() {
                    while current_path.len() - 1 >= level as usize {
                        current_path.pop();
                    }
                    current_path.push(text.to_string());

                    let path = &current_path.join("/");
                    let dir = Path::new(path);
                    fs::create_dir_all(dir)?;
                } else {
                    // Aquí puedes manejar el texto normal (por ejemplo, la línea con la URL)
                    println!("Texto normal: {}", text);
                    if let Some(caps) = re_list.captures(&text) {
                        let url = caps.get(1).unwrap().as_str();
                        let rest = caps.get(2).unwrap().as_str();
                        let tags: Vec<&str> = rest
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect();

                        println!("URL: {}", url);
                        println!("Etiquetas: {:?}", tags);

                        let path = &current_path.join("/");
                        let dir = Path::new(path);

                        println!("Descargando {}", url);

                        let output = Command::new("yt-dlp")
                            .args([
                                "--no-overwrites",
                                "-x",
                                "--audio-format",
                                "mp3",
                                "-o",
                                &(dir.join("%(title)s.%(ext)s").to_string_lossy()),
                                "--print",
                                "after_move:filepath",
                                url,
                            ])
                            .output()?;

                        let filename = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        let file_path = PathBuf::from(filename);

                        // Añadir etiquetas
                        let mut tag = id3::Tag::new();
                        tag.set_artist(current_path.last().unwrap());
                        tag.set_album(&current_path.join(" - "));
                        tag.add_comment(id3::frame::Comment {
                            lang: "ES".to_owned(),
                            description: "Custom tags".to_owned(),
                            text: tags.join(", "),
                        });

                        print!("path: {}", file_path.to_str().unwrap());
                        tag.write_to_path(&file_path, Version::Id3v24)?;
                        println!()
                    }
                }
            }
            _ => {}
        }
    }

    println!("\nFin!");

    Ok(())
}
