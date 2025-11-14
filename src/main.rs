use id3::{TagLike, Version};
use pulldown_cmark::{Event, Parser, Tag};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

type MusicFile = (String, Vec<String>, Vec<String>);

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <archivo.md>");
        std::process::exit(1);
    }

    let markdown_file = &args[1];

    let markdown = fs::read_to_string(markdown_file)?;

    let parser = Parser::new(&markdown);

    let mut current_path = vec!["./music".to_owned()];
    let mut current_heading_level = None;

    let mut links: Vec<MusicFile> = Vec::new();

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
                current_heading_level = Some(level);
            }
            Event::Text(text) => {
                if let Some(level) = current_heading_level.take() {
                    while current_path.len() - 1 >= level as usize {
                        current_path.pop();
                    }
                    current_path.push(text.to_string());

                    let path = &current_path.join("/");
                    let dir = Path::new(path);
                    fs::create_dir_all(dir)?;
                } else {
                    if let Some(caps) = re_list.captures(&text) {
                        let url = caps.get(1).unwrap().as_str();
                        let rest = caps.get(2).unwrap().as_str();
                        let tags: Vec<String> = rest
                            .split(',')
                            .map(|s| s.trim().to_owned())
                            .filter(|s| !s.is_empty())
                            .collect();

                        links.push((url.to_owned(), current_path.clone(), tags));
                    }
                }
            }
            _ => {}
        }
    }

    download_music(links);

    println!("\nFin!");

    Ok(())
}

fn download_music(links: Vec<MusicFile>) {
    println!("Inciando descargas...");

    links.par_iter().for_each(|(url, current_path, tags)| {
        let path = current_path.join("/");
        let dir = Path::new(&path);
        println!("Descargando {url} en {:?}", path);
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
            .output()
            .expect("Expected command completion");

        let filename = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let file_path = PathBuf::from(filename);

        let mut tag = id3::Tag::new();
        tag.set_artist(current_path.last().unwrap());
        tag.set_album(&current_path.join(" - "));
        tag.add_comment(id3::frame::Comment {
            lang: "ES".to_owned(),
            description: "Custom tags".to_owned(),
            text: tags.join(", "),
        });

        tag.write_to_path(&file_path, Version::Id3v24)
            .expect("Expected writing in path");

        println!(
            "Se ha descargado {}, con path {} y tags {:?}",
            url, path, tags
        );
    });
}
